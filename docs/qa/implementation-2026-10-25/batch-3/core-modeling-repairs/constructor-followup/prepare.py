from pathlib import Path
import hashlib,json
r=Path('docs/qa/implementation-2026-10-25/batch-3/core-modeling-repairs/constructor-followup')
base=Path('C:/github/qualiaDB/crates/qualia-core-db/src/specialized_libs/chemistry_modeling')
(r/'candidate').mkdir(exist_ok=True)
changes=[]
def stage(name,change):
 p=base/name; raw=p.read_bytes(); s=raw.decode(); crlf='\r\n' in s; s=s.replace('\r\n','\n'); s=change(s); b=s.replace('\n','\r\n').encode() if crlf else s.encode(); out=r/'candidate'/name;out.write_bytes(b)
 changes.append(dict(target=str(p),original_sha256=hashlib.sha256(raw).hexdigest(),candidate='candidate/'+name,candidate_sha256=hashlib.sha256(b).hexdigest()))
def types(s):
 s=s.replace('    /// Creates an empty molecule.','    /// Creates an empty molecule with zero-initialized, unmeasured properties.\n    /// The legacy local record key is preserved; it does not imply any structure.')
 s=s.replace('formula: "CH4".to_string(),','formula: String::new(),').replace('atoms: vec![Atom::new()],','atoms: Vec::new(),').replace('coordinates: vec![vec![0.0, 0.0, 0.0]; 5],','coordinates: Vec::new(),')
 s=s.replace('    /// Creates default molecular properties.','    /// Creates zero-initialized, unmeasured properties.\n    /// These values are placeholders, not measured or predicted chemical results.')
 s=s.replace('molecular_weight: 16.04,','molecular_weight: 0.0,')
 # Change only MolecularProperties energy, preserving other legacy property types.
 old='''            polarizability: 0.0,
            energy: -74.8,
''';assert s.count(old)==1;s=s.replace(old,old.replace('-74.8','0.0'))
 s+='''
#[cfg(test)]
mod constructor_tests {
    use super::*;

    #[test]
    fn new_molecule_is_empty_and_has_no_measured_properties() {
        let molecule = Molecule::new();
        assert_eq!(molecule.molecule_id, "mol_1"); // legacy local record key
        assert!(molecule.formula.is_empty());
        assert!(molecule.atoms.is_empty());
        assert!(molecule.bonds.is_empty());
        assert!(molecule.coordinates.is_empty());
        assert_eq!(molecule.properties.molecular_weight, 0.0);
        assert_eq!(molecule.properties.dipole_moment, 0.0);
        assert_eq!(molecule.properties.polarizability, 0.0);
        assert_eq!(molecule.properties.energy, 0.0);
    }

    #[test]
    fn new_molecular_properties_are_zero_initialized_not_chemical_results() {
        let properties = MolecularProperties::new();
        assert_eq!(properties.molecular_weight, 0.0);
        assert_eq!(properties.dipole_moment, 0.0);
        assert_eq!(properties.polarizability, 0.0);
        assert_eq!(properties.energy, 0.0);
    }
}
'''
 return s
def tests(s):
 old='''    // HONEST: an element beyond H/He (Molecule::new() is a carbon atom) still
    // reports NotImplemented rather than fabricating an energy.
    let unsupported =
        library.calculate_quantum_properties(Molecule::new(), QuantumMethodType::HartreeFock);'''
 new='''    // An explicit carbon atom is beyond H/He and must report NotImplemented.
    let unsupported =
        library.calculate_quantum_properties(carbon_molecule(), QuantumMethodType::HartreeFock);'''
 assert old in s;s=s.replace(old,new)
 s=s.replace('let molecule = Molecule::new(); // single C atom (mass 12.01)','let molecule = carbon_molecule(); // explicit single C atom (mass 12.01)')
 # The MD negative stays genuinely empty under the corrected public constructor.
 old='''    let empty = Molecule {
        atoms: Vec::new(),
        ..Molecule::new()
    };''';assert old in s;s=s.replace(old,'    let empty = Molecule::new();')
 helper='''
/// Explicit fixture; the empty constructor contributes no atoms or coordinates.
fn carbon_molecule() -> Molecule {
    let carbon = Atom::new();
    Molecule {
        formula: "C".to_string(),
        coordinates: vec![carbon.coordinates.clone()],
        atoms: vec![carbon],
        ..Molecule::new()
    }
}
'''
 return s.replace('use super::*;\n','use super::*;\n'+helper,1)
def properties(s):
 marker='''    ) -> Result<PredictedProperties, ChemistryError> {
        // Compute molecular descriptors from the molecule.''';assert s.count(marker)==1
 s=s.replace(marker,marker.replace('        // Compute','        self.validate_molecule(molecule)?;\n        // Compute'))
 s=s.replace('''        let mut m = Molecule::new();
        m.atoms.push''','''        let mut m = Molecule::new();
        m.formula = "H2O".to_string();
        m.atoms.push''',1)
 marker='''        m
    }

    #[test]
    fn initialized_predictor''';assert marker in s
 s=s.replace(marker,'''        m.coordinates = m.atoms.iter().map(|atom| atom.coordinates.clone()).collect();
        m
    }

    #[test]
    fn initialized_predictor''',1)
 s=s.replace('''        let mol = water_molecule();
        assert!(matches!(''','''        let mol = water_molecule();
        assert_eq!(mol.formula, "H2O");
        assert_eq!(mol.atoms.len(), 3);
        assert_eq!(mol.coordinates.len(), 3);
        assert!(!mol.atoms.iter().any(|atom| atom.element == "C"));
        assert!(matches!(''',1)
 end='''
    #[test]
    fn initialized_predictor_rejects_empty_and_formula_only_molecules() {
        let mut predictor = PropertyPredictor::new();
        predictor.initialize().unwrap();
        for formula in ["", "H2O"] {
            let mut molecule = Molecule::new();
            molecule.formula = formula.to_string();
            assert!(matches!(
                predictor.predict_from_molecule(&molecule, &[PropertyType::Density]),
                Err(ChemistryError::ValidationError(_))
            ));
        }
    }
'''
 assert s.endswith('}\n');return s[:-2]+end+'}\n'
stage('types.rs',types);stage('tests.rs',tests);stage('properties.rs',properties)
(r/'apply-manifest.json').write_text(json.dumps(dict(status='staged_only_not_applied',scope='Empty molecule constructor conformance, explicit caller fixtures, direct predictor empty validation',guard='Apply only matching original SHA; candidate SHA must match.',mutations=changes),indent=2)+'\n')
print(json.dumps(changes,indent=2))
