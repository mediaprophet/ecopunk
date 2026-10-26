use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use qualia_core_db::specialized_libs::chemistry_modeling::{ChemistryError, PhaseType, SimulationMetrics, QuantumMetrics, ReactionMetrics, PropertyMetrics};
#[path = "candidate/types.rs"] mod types;
use types::*;
#[path = "candidate/properties.rs"] mod properties;
use properties::*;
fn carbon_molecule() -> Molecule {
    let carbon = Atom::new();
    Molecule {
        formula: "C".to_string(),
        coordinates: vec![carbon.coordinates.clone()],
        atoms: vec![carbon],
        ..Molecule::new()
    }
}

#[test]
fn explicit_carbon_fixture_has_consistent_structure() {
    let carbon = carbon_molecule();
    assert_eq!(carbon.formula, "C");
    assert_eq!(carbon.atoms.len(), 1);
    assert_eq!(carbon.atoms[0].element, "C");
    assert_eq!(carbon.atoms[0].atomic_number, 6);
    assert_eq!(carbon.coordinates, vec![carbon.atoms[0].coordinates.clone()]);
    assert!(carbon.bonds.is_empty());
}
