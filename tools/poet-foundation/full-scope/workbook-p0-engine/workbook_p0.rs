//! Narrow native workbook derivative backed by GridCraft's existing session.
//! This is not an XLSX/ODS importer, a complete spreadsheet, or a rights record.
use gridcraft_engine::{core::CellRef, model::Workbook, DocState, Session};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};

pub(crate) const RESOURCE_KIND: &str = "poet-workbook-v1";
const PROFILE: &str = "poet.grid.native/1";
pub(crate) const MAX_RESOURCE_BYTES: usize = 128 * 1024;
const MAX_CELLS: usize = 256;
const MAX_INPUT_BYTES: usize = 256;
const SIDE: u32 = 64;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct StoredCell { cell: String, input: String }

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct WorkbookResource { profile: String, cells: Vec<StoredCell> }

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CellView { pub cell: String, pub input: String, pub display: String }

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkbookView { pub sheet: String, pub cells: Vec<CellView> }

fn check_cancelled(cancelled: &AtomicBool) -> Result<(), String> {
    if cancelled.load(Ordering::Acquire) { Err("Workbook operation cancelled".into()) } else { Ok(()) }
}

fn check_cell(cell: &str) -> Result<(), String> {
    let parsed = CellRef::parse(cell).ok_or("Workbook cell must be a valid explicit A1 address")?;
    if parsed.row >= SIDE || parsed.col >= SIDE || parsed.a1() != cell {
        return Err("Workbook cell must use canonical A1 notation within the 64 by 64 profile".into());
    }
    Ok(())
}

fn check_input(input: &str, allow_clear: bool) -> Result<(), String> {
    if input.len() > MAX_INPUT_BYTES || (!allow_clear && input.is_empty()) {
        return Err("Workbook input exceeds the profile limit or is empty".into());
    }
    // GridCraft interprets '=' and some '+'/'-' forms as formulas. P0 refuses
    // them all rather than attempting an unbounded or volatile recalculation.
    if matches!(input.trim_start().chars().next(), Some('=' | '+' | '-')) || input.chars().any(char::is_control) {
        return Err("Formulas, signed-leading inputs and control characters are unsupported by this workbook profile".into());
    }
    Ok(())
}

fn decode(bytes: &[u8], cancelled: &AtomicBool) -> Result<WorkbookResource, String> {
    check_cancelled(cancelled)?;
    if bytes.len() > MAX_RESOURCE_BYTES { return Err("Workbook resource exceeds the profile byte limit".into()); }
    let resource: WorkbookResource = serde_json::from_slice(bytes).map_err(|e| format!("Unsupported workbook resource: {e}"))?;
    if resource.profile != PROFILE || resource.cells.len() > MAX_CELLS {
        return Err("Unsupported workbook profile or cell count".into());
    }
    let mut previous: Option<&str> = None;
    for item in &resource.cells {
        check_cancelled(cancelled)?;
        check_cell(&item.cell)?;
        check_input(&item.input, false)?;
        if previous.is_some_and(|p| p >= item.cell.as_str()) {
            return Err("Workbook cells must be unique and ordered by canonical address".into());
        }
        previous = Some(&item.cell);
    }
    Ok(resource)
}

fn execute(session: &mut Session, cell: &str, input: &str) -> Result<String, String> {
    let result = session.execute("cell.set", serde_json::json!({"sheet":0,"cell":cell,"input":input,"array":false}))
        .map_err(|e| format!("GridCraft cell edit refused: {e}"))?;
    result.get("value").and_then(serde_json::Value::as_str).map(str::to_owned)
        .ok_or_else(|| "GridCraft did not return a cell display value".into())
}

fn replay(resource: &WorkbookResource, cancelled: &AtomicBool) -> Result<(Session, Vec<CellView>), String> {
    let mut session = Session::new();
    session.add_document(DocState::new(Workbook::new(), None, "Native workbook".into()));
    let mut cells = Vec::with_capacity(resource.cells.len());
    for item in &resource.cells {
        check_cancelled(cancelled)?;
        let display = execute(&mut session, &item.cell, &item.input)?;
        cells.push(CellView { cell: item.cell.clone(), input: item.input.clone(), display });
    }
    check_cancelled(cancelled)?;
    Ok((session, cells))
}

/// Creates a blank, restricted native profile. It makes no provenance claim.
pub(crate) fn blank() -> Vec<u8> { br#"{"profile":"poet.grid.native/1","cells":[]}"#.to_vec() }

/// Reads only this profile, executing bounded explicit GridCraft cell commands.
pub(crate) fn inspect(bytes: &[u8], cancelled: &AtomicBool) -> Result<WorkbookView, String> {
    let resource = decode(bytes, cancelled)?;
    let (_, cells) = replay(&resource, cancelled)?;
    Ok(WorkbookView { sheet: "Sheet1".into(), cells })
}

/// Builds a detached candidate; the caller must perform authority, digest and
/// source checks and atomically commit it through the checkpoint owner.
pub(crate) fn edit(bytes: &[u8], sheet: &str, cell: &str, input: &str, cancelled: &AtomicBool) -> Result<Vec<u8>, String> {
    if sheet != "Sheet1" { return Err("Workbook profile supports only explicit Sheet1".into()); }
    check_cell(cell)?;
    check_input(input, true)?;
    let mut resource = decode(bytes, cancelled)?;
    let (mut session, _) = replay(&resource, cancelled)?;
    check_cancelled(cancelled)?;
    execute(&mut session, cell, input)?;
    match resource.cells.binary_search_by(|item| item.cell.as_str().cmp(cell)) {
        Ok(index) if input.is_empty() => { resource.cells.remove(index); }
        Ok(index) => resource.cells[index].input = input.to_string(),
        Err(_) if input.is_empty() => {}
        Err(index) => {
            if resource.cells.len() >= MAX_CELLS { return Err("Workbook cell count limit reached".into()); }
            resource.cells.insert(index, StoredCell { cell: cell.into(), input: input.into() });
        }
    }
    check_cancelled(cancelled)?;
    let encoded = serde_json::to_vec(&resource).map_err(|e| format!("Workbook encoding failed: {e}"))?;
    if encoded.len() > MAX_RESOURCE_BYTES { return Err("Workbook encoded resource exceeds the profile byte limit".into()); }
    Ok(encoded)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn native_workbook_edits_replay_and_clear_through_gridcraft() {
        let cancel = AtomicBool::new(false);
        let first = edit(&blank(), "Sheet1", "A1", "42", &cancel).unwrap();
        let second = edit(&first, "Sheet1", "B2", "hello", &cancel).unwrap();
        let view = inspect(&second, &cancel).unwrap();
        assert_eq!(view.cells, vec![CellView { cell:"A1".into(), input:"42".into(), display:"42".into() }, CellView { cell:"B2".into(), input:"hello".into(), display:"hello".into() }]);
        let cleared = edit(&second, "Sheet1", "A1", "", &cancel).unwrap();
        assert_eq!(inspect(&cleared, &cancel).unwrap().cells.len(), 1);
        assert_eq!(inspect(&first, &cancel).unwrap().cells.len(), 1);
    }
    #[test]
    fn workbook_profile_refuses_unsupported_or_ambiguous_content() {
        let cancel = AtomicBool::new(false);
        for bytes in [br#"{"profile":"poet.grid.native/1","cells":[],"unknown":1}"#.as_slice(), br#"{"profile":"other","cells":[]}"#, br#"{"profile":"poet.grid.native/1","cells":[{"cell":"A1","input":"=NOW()"}]}"#, br#"{"profile":"poet.grid.native/1","cells":[{"cell":"A1","input":"x"},{"cell":"A1","input":"y"}]}"#] { assert!(inspect(bytes, &cancel).is_err()); }
        for (sheet, cell, input) in [("", "A1", "x"), ("Sheet1", "a1", "x"), ("Sheet1", "BM1", "x"), ("Sheet1", "A1", "=1+1"), ("Sheet1", "A1", "+A1"), ("Sheet1", "A1", "  =NOW()"), ("Sheet1", "A1", "x\ny")] { assert!(edit(&blank(), sheet, cell, input, &cancel).is_err()); }
        let cancelled = AtomicBool::new(true);
        assert!(inspect(&blank(), &cancelled).is_err());
        assert!(edit(&blank(), "Sheet1", "A1", "x", &cancelled).is_err());
    }
}
