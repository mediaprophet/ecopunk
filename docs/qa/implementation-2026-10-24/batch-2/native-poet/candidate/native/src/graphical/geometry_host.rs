//! Explicit native geometry invoke profile over the actual Qualia host.
use qualia_core_db::poet_host::PoetSnapshot;
use vibe::{DiagCode, Diagnostic, Host, Span, Value};

pub struct GeometryHost {
    inner: PoetSnapshot,
    max_vertices: usize,
    grants: Vec<String>,
    remaining_work: usize,
    remaining_dispatches: usize,
}
impl Default for GeometryHost {
    fn default() -> Self {
        Self::new(Vec::new(), 200_000)
    }
}
impl GeometryHost {
    /// Host-selected working set, not a master-quality ceiling or script grant.
    pub fn new(grants: Vec<String>, max_vertices: usize) -> Self {
        Self {
            inner: PoetSnapshot::default(),
            max_vertices,
            grants,
            remaining_work: max_vertices.saturating_mul(2),
            remaining_dispatches: 32,
        }
    }
    pub(crate) fn actual_grants(&self) -> &[String] {
        &self.grants
    }
    pub(crate) fn begin_submission(&mut self) {
        self.remaining_work = self.max_vertices.saturating_mul(2);
        self.remaining_dispatches = 32;
    }
    pub(crate) fn for_rebuild(&self, total_work: usize, dispatches: usize) -> Self {
        let mut next = Self::new(self.grants.clone(), self.max_vertices);
        next.remaining_work = total_work;
        next.remaining_dispatches = dispatches;
        next
    }
    fn admit(&mut self, id: &str, args: &Value, span: Span) -> Result<(), Diagnostic> {
        let Value::Record(record) = args else {
            return Err(denied(span));
        };
        let length = |key: &str| match record.get(key) {
            Some(Value::List(values)) => Some(values.len()),
            _ => None,
        };
        let vertices = match id {
            "ComputationalGeometry.extrude_profile" => length("axis")
                .zip(length("section"))
                .and_then(|(axis, section)| axis.checked_mul(section))
                .and_then(|count| count.checked_add(2)),
            "ComputationalGeometry.swung_surface" => {
                let m = match record.get("profiles") {
                    Some(Value::List(rows)) if rows.len() <= 256 => match rows.first() {
                        Some(Value::List(row)) => Some(row.len() / 2),
                        _ => None,
                    },
                    _ => None,
                };
                let slices = match record.get("slices") {
                    Some(Value::U64(v)) => usize::try_from(*v).ok(),
                    Some(Value::I64(v)) if *v >= 0 => usize::try_from(*v).ok(),
                    _ => None,
                };
                m.zip(slices).and_then(|(m, slices)| m.checked_mul(slices))
            }
            _ => None,
        };
        if self.remaining_dispatches == 0
            || vertices.is_none_or(|count| {
                count == 0 || count > self.max_vertices || count > self.remaining_work
            })
        {
            return Err(Diagnostic::new(
                DiagCode::E400,
                span,
                "geometry exceeds host-selected preview working set",
            ));
        }
        self.remaining_work -= vertices.expect("geometry work admitted");
        self.remaining_dispatches -= 1;
        Ok(())
    }
}

fn denied(span: Span) -> Diagnostic {
    Diagnostic::new(
        DiagCode::E300,
        span,
        "operation is outside the admitted native geometry preview profile",
    )
}

impl Host for GeometryHost {
    fn graph_query(&mut self, _: &[Value], _: u64, span: Span) -> Result<Value, Diagnostic> {
        Err(denied(span))
    }
    fn graph_stage(&mut self, _: &Value, span: Span) -> Result<Value, Diagnostic> {
        Err(denied(span))
    }
    fn graph_commit(&mut self, span: Span) -> Result<Value, Diagnostic> {
        Err(denied(span))
    }
    fn aura_validate(&mut self, _: &Value, _: &Value, span: Span) -> Result<Value, Diagnostic> {
        Err(denied(span))
    }
    fn pulse_publish(&mut self, _: &str, _: &Value, span: Span) -> Result<Value, Diagnostic> {
        Err(denied(span))
    }
    fn capability_invoke(
        &mut self,
        id: &str,
        args: &Value,
        span: Span,
    ) -> Result<Value, Diagnostic> {
        // The canonical checker separately checks the invocation-style grant.
        // Runtime authority always checks the actual target, including dynamic IDs.
        if !self.grants.iter().any(|grant| grant == id) {
            return Err(denied(span));
        }
        match id {
            "ComputationalGeometry.extrude_profile" | "ComputationalGeometry.swung_surface" => {
                self.admit(id, args, span)?;
                self.inner.capability_invoke(id, args, span)
            }
            _ => Err(denied(span)),
        }
    }
}

/// Source-only diagnostic recipe, not a production organism or accepted model.
/// It uses the real editable section/path operator, never primitive counts.
pub const ORGANIC_SURFACE_SOURCE: &str = r#"requires [ capability("capability.invoke"), capability("ComputationalGeometry.extrude_profile") ];
capability.invoke("ComputationalGeometry.extrude_profile", {
  axis: [[0.0, 0.0, -1.0], [0.12, 0.06, -0.4], [0.18, 0.02, 0.3], [0.1, 0.0, 1.0]],
  section: [[-0.20, -0.10], [0.18, -0.12], [0.24, 0.05], [0.02, 0.20], [-0.18, 0.10]],
  closed: true, caps: true
});"#;

#[cfg(test)]
mod tests {
    use super::*;
    use vibe::session::{SessionLimits, SourceSession};
    #[test]
    fn actual_operation_grants_guard_dynamic_targets_at_runtime() {
        let grants = vec![
            "capability.invoke".into(),
            "ComputationalGeometry.extrude_profile".into(),
        ];
        let mut session = SourceSession::new(grants.clone(), SessionLimits::default());
        let mut host = GeometryHost::new(grants, 200_000);
        let source = r#"requires [capability("capability.invoke")];
            let op = "ComputationalGeometry.swung_surface";
            capability.invoke(op, {profiles:[[1.0,0.0,1.0,1.0],[1.1,0.0,1.1,1.0]], angles:[0.0,3.14], slices:8});"#;
        let error = session
            .submit(source, &mut host, vibe::Budget::default())
            .unwrap_err();
        assert_eq!(error.diagnostic.code, DiagCode::E300);
        assert!(error
            .diagnostic
            .message
            .contains("outside the admitted native geometry preview profile"));
        assert!(error.execution_attempted);
        assert_eq!(session.revision(), 0);
    }
    #[test]
    fn granted_source_dispatches_real_native_geometry_kernel() {
        let grants = vec![
            "capability.invoke".into(),
            "ComputationalGeometry.extrude_profile".into(),
        ];
        let mut session = SourceSession::new(grants.clone(), SessionLimits::default());
        let mut host = GeometryHost::new(grants, 200_000);
        let result = session
            .submit(ORGANIC_SURFACE_SOURCE, &mut host, vibe::Budget::default())
            .unwrap();
        let Value::Record(mesh) = result.value else {
            panic!("actual geometry result record")
        };
        assert!(
            matches!(mesh.get("vertices"), Some(Value::List(vertices)) if !vertices.is_empty())
        );
        assert!(
            matches!(mesh.get("triangles"), Some(Value::List(triangles)) if !triangles.is_empty())
        );
        assert_eq!(result.revision, 1);
    }
}
