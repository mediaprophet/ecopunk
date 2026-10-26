mod chemistry {
use qualia_core_db::specialized_libs::chemistry_modeling::*;
use std::collections::HashMap;
fn water_molecule() -> Molecule {
    let mut m = Molecule::new();
    m.atoms.push(Atom {
        atom_id: "O1".to_string(),
        element: "O".to_string(),
        atomic_number: 8,
        mass: 15.999,
        charge: 0.0,
        coordinates: vec![0.0, 0.0, 0.0],
    });
    m.atoms.push(Atom {
        atom_id: "H1".to_string(),
        element: "H".to_string(),
        atomic_number: 1,
        mass: 1.008,
        charge: 0.0,
        coordinates: vec![0.0, 0.757, 0.586],
    });
    m.atoms.push(Atom {
        atom_id: "H2".to_string(),
        element: "H".to_string(),
        atomic_number: 1,
        mass: 1.008,
        charge: 0.0,
        coordinates: vec![0.0, -0.757, 0.586],
    });
    m
}
#[test]
fn initialized_predictor_uses_registered_group_contribution_formulas() {
    let uninitialized = PropertyPredictor::new();
    let mol = water_molecule();
    assert!(matches!(
        uninitialized.predict_from_molecule(&mol, &[PropertyType::BoilingPoint]),
        Err(ChemistryError::NotImplemented(_))
    ));

    let mut predictor = PropertyPredictor::new();
    predictor.initialize().unwrap();
    let registered = predictor.list_properties();
    assert!(registered.iter().any(|name| name == "boiling_point"));
    assert!(registered.iter().any(|name| name == "density"));
    assert!(registered.iter().any(|name| name == "heat_capacity"));

    // These assertions pin the implemented group-contribution equations.
    // They are formula/dispatch checks, not claims of empirical accuracy.
    let descriptors = HashMap::from([
        ("C".to_string(), 0.0),
        ("H".to_string(), 2.0),
        ("O".to_string(), 1.0),
        ("N".to_string(), 0.0),
        ("ring".to_string(), 0.0),
    ]);
    assert!((predictor.predict("boiling_point", &descriptors).unwrap() - 253.96).abs() < 1e-12);
    assert!(matches!(
        predictor.predict("unknown_property", &descriptors),
        Err(ChemistryError::NotImplemented(_))
    ));

    let res = predictor
        .predict_from_molecule(
            &mol,
            &[
                PropertyType::BoilingPoint,
                PropertyType::Density,
                PropertyType::HeatCapacity,
            ],
        )
        .unwrap();
    assert!(res.properties.contains_key("boiling_point"));
    assert!(res.properties.contains_key("density"));
    assert!(res.properties.contains_key("heat_capacity"));
    // H₂O descriptors yield the implemented equation values exactly:
    // density = .85 + .0018·18.015 + .08·1; heat capacity = 25 + 7.5·2 + 18·1.
    assert!((res.properties["density"] - 0.962427).abs() < 1e-12);
    assert!((res.properties["heat_capacity"] - 58.0).abs() < 1e-12);
}
}
mod finance {
use qualia_core_db::specialized_libs::financial_modeling::*;
use std::collections::HashMap;
#[test]
fn ingest_from_feed_fails_closed_without_cached_prices() {
    let mut manager = AssetManager::new();
    manager.register_price_feed(PriceFeed {
        feed_id: "seeded_feed".to_string(),
        feed_name: "no cache".to_string(),
        feed_type: FeedType::Historical,
        update_frequency: 86400,
        data_quality: DataQuality::new(),
        last_update: 0,
        asset_id: "B".to_string(),
        cached_prices: Vec::new(),
    });

    assert!(matches!(
        manager.ingest_from_feed("B"),
        Err(FinancialError::DataError(_))
    ));
    assert!(manager.get_price_history("B").is_none());
}
}
mod engineering {
use qualia_core_db::specialized_libs::engineering_analysis::*;
use std::collections::HashMap;
fn prismatic_model(load: f64) -> EngineeringModel {
    // b=h=0.1 (area 0.01), L=2, steel (E=200000, rho=7850, yield 250).
    let mut model = EngineeringModel::new();
    model.geometry.dimensions = vec![0.1, 0.1, 2.0];
    model.materials.insert("steel".to_string(), Material::new());
    model.loads.push(Load {
        load_id: "F".to_string(),
        load_type: LoadType::Force,
        load_magnitude: load,
        load_direction: vec![1.0, 0.0, 0.0],
        application_point: vec![0.0, 0.0, 0.0],
    });
    model
}
fn si_prismatic_model(load: f64) -> EngineeringModel {
    // Consistent SI fixture: 0.1 m × 0.1 m × 2 m steel bar; N, Pa, kg/m³.
    let mut model = prismatic_model(load);
    let material = &mut model
        .materials
        .get_mut("steel")
        .unwrap()
        .material_properties;
    material.youngs_modulus = 200.0e9;
    material.yield_strength = 250.0e6;
    material.density = 7850.0;
    model
}
#[test]
fn test_mechanical_analysis() {
    let mut library = EngineeringAnalysisLibrary::new();
    library.initialize().unwrap();

    // SI prismatic steel bar: A=0.01 m², L=2 m, E=200 GPa, F=200 N.
    // The closed-form static tip displacement is F·L/(A·E)=2e-7 m.
    // The undamped step-load dynamic peak approaches twice that static value.
    let stat = library
        .perform_mechanical_analysis(si_prismatic_model(200.0), AnalysisType::LinearStatic)
        .unwrap()
        .result;
    assert!((stat.displacement_field[0] - 2.0e-7).abs() < 1e-15);
    assert!((stat.stress_field[0] - 20_000.0).abs() < 1e-9);

    let dynamic = library
        .perform_mechanical_analysis(si_prismatic_model(200.0), AnalysisType::LinearDynamic)
        .unwrap()
        .result;
    let dynamic_amplification = dynamic.displacement_field[0] / stat.displacement_field[0];
    assert!(
        (dynamic_amplification - 2.0).abs() < 0.02,
        "step-load dynamic amplification = {dynamic_amplification}"
    );

    let mut missing_material = si_prismatic_model(200.0);
    missing_material.materials.clear();
    assert!(matches!(
        library.perform_mechanical_analysis(missing_material, AnalysisType::LinearStatic),
        Err(EngineeringError::InsufficientData(_))
    ));
}
#[test]
fn structural_vibration_facade_matches_prismatic_closed_form_and_rejects_invalid_density() {
    let mut library = EngineeringAnalysisLibrary::new();
    library.initialize().unwrap();
    let result = library
        .perform_structural_analysis(si_prismatic_model(200.0), AnalysisType::Vibration)
        .unwrap()
        .result;
    assert_eq!(result.analysis_type, AnalysisType::Vibration);

    let area: f64 = 0.1 * 0.1;
    let length: f64 = 2.0;
    let si_model = si_prismatic_model(200.0);
    let material = si_model.materials.get("steel").unwrap();
    let e = material.material_properties.youngs_modulus;
    let density = material.material_properties.density;
    let expected_omega = ((e * area / length) / (density * area * length / 2.0)).sqrt();
    assert!((result.stress_field[0] - expected_omega).abs() < 1e-9);
    assert!(
        (result.displacement_field[0] - expected_omega / (2.0 * std::f64::consts::PI)).abs() < 1e-9
    );
    // The solver reports its first mode in reaction_forces, normalized to max |φ|=1.
    assert_eq!(result.reaction_forces.len(), 1);
    assert!(result.reaction_forces.iter().all(|value| value.is_finite()));
    assert!((result.reaction_forces[0].abs() - 1.0).abs() < 1e-12);

    let mut invalid_density = si_prismatic_model(200.0);
    invalid_density
        .materials
        .get_mut("steel")
        .unwrap()
        .material_properties
        .density = 0.0;
    assert!(matches!(
        library.perform_structural_analysis(invalid_density, AnalysisType::Vibration),
        Err(EngineeringError::InsufficientData(_))
    ));
}
}