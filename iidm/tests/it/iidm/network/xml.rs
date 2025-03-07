use iidm::xml::*;

const NETWORK_XML: &str = "tests/data/network.xiidm";

#[test]
fn test_deserialize_from_xml() -> Result<(), Box<dyn std::error::Error>> {
    let test_network = std::fs::read_to_string(NETWORK_XML)?;
    let network: Network = quick_xml::de::from_str(&test_network)?;

    // Vérification des informations de base du réseau
    assert_eq!(network.id, "sim1");
    assert_eq!(network.forecast_distance, 0);
    assert_eq!(network.source_format, "test");
    assert_eq!(network.minimum_validation_level, "STEADY_STATE_HYPOTHESIS");

    // Vérification du nombre de postes
    assert_eq!(network.substations.len(), 2);

    // Vérification du premier poste (P1)
    let substation1 = &network.substations[0];
    assert_eq!(substation1.id, "P1");
    assert_eq!(substation1.country, "FR");
    assert_eq!(substation1.tso, "RTE");
    assert_eq!(substation1.geographical_tags.get(0).unwrap(), "A");

    // Vérification des niveaux de tension du poste P1
    assert_eq!(substation1.voltage_levels.len(), 2);

    // Vérification du niveau de tension VLGEN
    let vlgen = &substation1.voltage_levels[0];
    assert_eq!(vlgen.id, "VLGEN");
    assert_eq!(vlgen.nominal_v, 24.0);
    assert_eq!(vlgen.topology_kind, TopologyKind::BusBreaker);

    // Vérification de la topologie bus-breaker de VLGEN
    assert!(vlgen.bus_breaker_topology.is_some());
    let bus_breaker = vlgen.bus_breaker_topology.as_ref().unwrap();
    assert_eq!(bus_breaker.buses.len(), 1);
    assert_eq!(bus_breaker.buses[0].id, "NGEN");

    // Vérification des générateurs de VLGEN
    assert_eq!(vlgen.generators.len(), 1);
    let gen = &vlgen.generators[0];
    assert_eq!(gen.id, "GEN");
    assert_eq!(gen.energy_source, EnergySource::OTHER);
    assert_eq!(gen.min_p, -9999.99);
    assert_eq!(gen.max_p, 9999.99);
    assert!(gen.voltage_regulator_on);
    assert_eq!(gen.target_p, 607.0);
    assert_eq!(gen.target_v, 24.5);
    assert_eq!(gen.target_q, 301.0);
    assert_eq!(gen.bus, "NGEN");
    assert_eq!(gen.connectable_bus, "NGEN");

    // Vérification des limites réactives du générateur
    assert!(gen.reactive_capability_curve.is_none());
    assert!(gen.min_max_reactive_limits.is_some());
    let limits = gen.min_max_reactive_limits.as_ref().unwrap();
    assert_eq!(limits.min_q, -9999.99);
    assert_eq!(limits.max_q, 9999.99);

    // Vérification du niveau de tension VLHV1
    let vlhv1 = &substation1.voltage_levels[1];
    assert_eq!(vlhv1.id, "VLHV1");
    assert_eq!(vlhv1.nominal_v, 380.0);
    assert_eq!(vlhv1.topology_kind, TopologyKind::BusBreaker);

    // Vérification des transformateurs deux enroulements du poste P1
    assert_eq!(substation1.two_windings_transformers.len(), 1);
    let transfo1 = &substation1.two_windings_transformers[0];
    assert_eq!(transfo1.id, "NGEN_NHV1");
    assert_eq!(transfo1.r, 0.26658461538461536);
    assert_eq!(transfo1.x, 11.104492831516762);
    assert_eq!(transfo1.g, 0.0);
    assert_eq!(transfo1.b, 0.0);
    assert_eq!(transfo1.rated_u1, 24.0);
    assert_eq!(transfo1.rated_u2, 400.0);
    assert_eq!(transfo1.voltage_level_id1, "VLGEN");
    assert_eq!(transfo1.bus1, "NGEN");
    assert_eq!(transfo1.connectable_bus1, "NGEN");
    assert_eq!(transfo1.voltage_level_id2, "VLHV1");
    assert_eq!(transfo1.bus2, "NHV1");
    assert_eq!(transfo1.connectable_bus2, "NHV1");

    // Vérification du deuxième poste (P2)
    let substation2 = &network.substations[1];
    assert_eq!(substation2.id, "P2");
    assert_eq!(substation2.country, "FR");
    assert_eq!(substation2.tso, "RTE");
    assert_eq!(substation2.geographical_tags.get(0).unwrap(), "B");

    // Vérification des niveaux de tension du poste P2
    assert_eq!(substation2.voltage_levels.len(), 2);

    // Vérification du niveau de tension VLLOAD
    let vlload = &substation2.voltage_levels[1];
    assert_eq!(vlload.id, "VLLOAD");
    assert_eq!(vlload.nominal_v, 150.0);

    // Vérification des charges dans VLLOAD
    assert_eq!(vlload.loads.len(), 1);
    let load = &vlload.loads[0];
    assert_eq!(load.id, "LOAD");
    assert_eq!(load.load_type, LoadType::UNDEFINED);
    assert_eq!(load.p0, 600.0);
    assert_eq!(load.q0, 200.0);
    assert_eq!(load.bus, "NLOAD");
    assert_eq!(load.connectable_bus, "NLOAD");

    // Vérification des transformateurs deux enroulements du poste P2
    assert_eq!(substation2.two_windings_transformers.len(), 1);
    let transfo2 = &substation2.two_windings_transformers[0];
    assert_eq!(transfo2.id, "NHV2_NLOAD");
    assert_eq!(transfo2.r, 0.04724999999999999);
    assert_eq!(transfo2.x, 4.049724365620455);

    // Vérification du changeur de prise en charge du transformateur
    assert!(transfo2.ratio_tap_changer.is_some());
    let rtc = transfo2.ratio_tap_changer.as_ref().unwrap();
    assert!(rtc.regulating);
    assert_eq!(rtc.low_tap_position, 0);
    assert_eq!(rtc.tap_position, 1);
    assert_eq!(rtc.target_deadband, 0.0);
    assert!(rtc.load_tap_changing_capabilities);
    assert_eq!(rtc.regulation_mode, RatioRegulationMode::VOLTAGE);
    assert_eq!(rtc.regulation_value, 158.0);

    // Vérification de la référence de terminal du RTC
    assert_eq!(rtc.terminal_ref.id, "NHV2_NLOAD");
    assert_eq!(rtc.terminal_ref.side, Side::TWO);

    // Vérification des étapes du RTC
    assert_eq!(rtc.steps.len(), 3);
    assert_eq!(rtc.steps[0].rho, 0.8505666905244191);
    assert_eq!(rtc.steps[1].rho, 1.0006666666666666);
    assert_eq!(rtc.steps[2].rho, 1.150766642808914);

    // Vérification des lignes
    assert_eq!(network.lines.len(), 2);

    // Vérification de la première ligne
    let line1 = &network.lines[0];
    assert_eq!(line1.id, "NHV1_NHV2_1");
    assert_eq!(line1.r, 3.0);
    assert_eq!(line1.x, 33.0);
    assert_eq!(line1.g1, 0.0);
    assert_eq!(line1.b1, 1.93E-4);
    assert_eq!(line1.g2, 0.0);
    assert_eq!(line1.b2, 1.93E-4);
    assert_eq!(line1.voltage_level_id1, "VLHV1");
    assert_eq!(line1.bus1, "NHV1");
    assert_eq!(line1.connectable_bus1, "NHV1");
    assert_eq!(line1.voltage_level_id2, "VLHV2");
    assert_eq!(line1.bus2, "NHV2");
    assert_eq!(line1.connectable_bus2, "NHV2");

    // Vérification de la deuxième ligne
    let line2 = &network.lines[1];
    assert_eq!(line2.id, "NHV1_NHV2_2");
    assert_eq!(line2.r, 3.0);
    assert_eq!(line2.x, 33.0);
    assert_eq!(line2.g1, 0.0);
    assert_eq!(line2.b1, 1.93E-4);
    assert_eq!(line2.g2, 0.0);
    assert_eq!(line2.b2, 1.93E-4);

    Ok(())
}
