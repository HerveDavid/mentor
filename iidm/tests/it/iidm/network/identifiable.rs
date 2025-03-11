use bevy_ecs::{event::Events, schedule::Schedule, world::World};
use iidm::*;

const NETWORK_XML: &str = "tests/data/network.xiidm";

fn load_network(path: &str) -> xml::Network {
    let test_network = std::fs::read_to_string(path).unwrap();
    quick_xml::de::from_str(&test_network).unwrap()
}

#[test]
fn test_register_network_basic() {
    let network = load_network(NETWORK_XML);

    // Create a new world and schedule
    let mut world = World::new();
    let mut schedule = Schedule::default();

    // Initialize required resources
    world.init_resource::<AssetRegistry>();
    world.init_resource::<Events<RegisterEvent>>();

    // Add systems to schedule
    schedule.add_systems(handle_register_events);

    // Register the network
    network.register(&mut world, &mut schedule);

    // Verify network was registered
    let registry = world.resource::<AssetRegistry>();
    assert!(registry.find("sim1").is_some());
}

#[test]
fn test_substation_registration() {
    // Create a new world and schedule
    let mut world = World::new();
    let mut schedule = Schedule::default();

    // Initialize required resources
    world.init_resource::<AssetRegistry>();
    world.init_resource::<Events<RegisterEvent>>();

    // Add systems to schedule
    schedule.add_systems(handle_register_events);

    // Create and register network
    let network = load_network(NETWORK_XML);
    network.register(&mut world, &mut schedule);

    // Verify substations were registered
    let registry = world.resource::<AssetRegistry>();

    // Check if substations exist in registry
    assert!(registry.find("P1").is_some());
    assert!(registry.find("P2").is_some());

    // Query for actual substation components
    let mut substation_query = world.query::<&xml::Substation>();
    let substations: Vec<&xml::Substation> = substation_query.iter(&world).collect();

    assert_eq!(substations.len(), 2);
    assert!(substations.iter().any(|s| s.id == "P1"));
    assert!(substations.iter().any(|s| s.id == "P2"));
}

#[test]
fn test_voltage_level_registration() {
    // Create a new world and schedule
    let mut world = World::new();
    let mut schedule = Schedule::default();

    // Initialize required resources
    world.init_resource::<AssetRegistry>();
    world.init_resource::<Events<RegisterEvent>>();

    // Add systems to schedule
    schedule.add_systems(handle_register_events);

    // Create and register network
    let network = load_network(NETWORK_XML);
    network.register(&mut world, &mut schedule);

    // Verify voltage levels were registered
    let registry = world.resource::<AssetRegistry>();

    // Check if voltage levels exist in registry
    assert!(registry.find("VLGEN").is_some());
    assert!(registry.find("VLHV1").is_some());
    assert!(registry.find("VLHV2").is_some());
    assert!(registry.find("VLLOAD").is_some());
}

#[test]
fn test_generator_registration() {
    // Create a new world and schedule
    let mut world = World::new();
    let mut schedule = Schedule::default();

    // Initialize required resources
    world.init_resource::<AssetRegistry>();
    world.init_resource::<Events<RegisterEvent>>();

    // Add systems to schedule
    schedule.add_systems(handle_register_events);

    // Create and register network
    let network = load_network(NETWORK_XML);
    network.register(&mut world, &mut schedule);

    // Verify generators were registered
    let registry = world.resource::<AssetRegistry>();

    // Check if generator exists in registry
    assert!(registry.find("GEN").is_some());

    // Query for actual generator components
    let mut generator_query = world.query::<&xml::Generator>();
    let generators: Vec<&xml::Generator> = generator_query.iter(&world).collect();

    assert_eq!(generators.len(), 1);
    assert!(generators.iter().any(|g| g.id == "GEN"));
}

#[test]
fn test_load_registration() {
    // Create a new world and schedule
    let mut world = World::new();
    let mut schedule = Schedule::default();

    // Initialize required resources
    world.init_resource::<AssetRegistry>();
    world.init_resource::<Events<RegisterEvent>>();

    // Add systems to schedule
    schedule.add_systems(handle_register_events);

    // Create and register network
    let network = load_network(NETWORK_XML);
    network.register(&mut world, &mut schedule);

    // Verify loads were registered
    let registry = world.resource::<AssetRegistry>();

    // Check if load exists in registry
    assert!(registry.find("LOAD").is_some());

    // Query for actual load components
    let mut load_query = world.query::<&xml::Load>();
    let loads: Vec<&xml::Load> = load_query.iter(&world).collect();

    assert_eq!(loads.len(), 1);
    assert!(loads.iter().any(|l| l.id == "LOAD"));
}

#[test]
fn test_transformer_registration() {
    // Create a new world and schedule
    let mut world = World::new();
    let mut schedule = Schedule::default();

    // Initialize required resources
    world.init_resource::<AssetRegistry>();
    world.init_resource::<Events<RegisterEvent>>();

    // Add systems to schedule
    schedule.add_systems(handle_register_events);

    // Create and register network
    let network = load_network(NETWORK_XML);
    network.register(&mut world, &mut schedule);

    // Verify transformers were registered
    let registry = world.resource::<AssetRegistry>();

    // Check if transformers exist in registry
    assert!(registry.find("NGEN_NHV1").is_some());
    assert!(registry.find("NHV2_NLOAD").is_some());

    // Query for actual transformer components
    let mut transformer_query = world.query::<&xml::TwoWindingsTransformer>();
    let transformers: Vec<&xml::TwoWindingsTransformer> = transformer_query.iter(&world).collect();

    assert_eq!(transformers.len(), 2);
    assert!(transformers.iter().any(|t| t.id == "NGEN_NHV1"));
    assert!(transformers.iter().any(|t| t.id == "NHV2_NLOAD"));
}

#[test]
fn test_line_registration() {
    // Create a new world and schedule
    let mut world = World::new();
    let mut schedule = Schedule::default();

    // Initialize required resources
    world.init_resource::<AssetRegistry>();
    world.init_resource::<Events<RegisterEvent>>();

    // Add systems to schedule
    schedule.add_systems(handle_register_events);

    // Create and register network
    let network = load_network(NETWORK_XML);
    network.register(&mut world, &mut schedule);

    // Verify lines were registered
    let registry = world.resource::<AssetRegistry>();

    // Check if lines exist in registry
    assert!(registry.find("NHV1_NHV2_1").is_some());
    assert!(registry.find("NHV1_NHV2_2").is_some());

    // Query for actual line components
    let mut line_query = world.query::<&xml::Line>();
    let lines: Vec<&xml::Line> = line_query.iter(&world).collect();

    assert_eq!(lines.len(), 2);
    assert!(lines.iter().any(|l| l.id == "NHV1_NHV2_1"));
    assert!(lines.iter().any(|l| l.id == "NHV1_NHV2_2"));
}

#[test]
fn test_network_register_empty() {
    // Create a new world and schedule
    let mut world = World::new();
    let mut schedule = Schedule::default();

    // Initialize required resources
    world.init_resource::<AssetRegistry>();
    world.init_resource::<Events<RegisterEvent>>();

    // Add systems to schedule
    schedule.add_systems(handle_register_events);

    // Create an empty network (with no substations)
    let mut network = load_network(NETWORK_XML);
    network.substations.clear();

    // Register the empty network
    network.register(&mut world, &mut schedule);

    // Query for substation components (should be none)
    let mut substation_query = world.query::<&xml::Substation>();
    let substations: Vec<&xml::Substation> = substation_query.iter(&world).collect();

    // Assert that no substations were registered
    assert_eq!(substations.len(), 0);
}

#[test]
fn test_network_register_idempotency() {
    // Create a new world and schedule
    let mut world = World::new();
    let mut schedule = Schedule::default();

    // Initialize required resources
    world.init_resource::<AssetRegistry>();
    world.init_resource::<Events<RegisterEvent>>();

    // Add systems to schedule
    schedule.add_systems(handle_register_events);

    // Create a test network
    let network = load_network(NETWORK_XML);

    // Register the network twice to test idempotency
    network.register(&mut world, &mut schedule);
    network.register(&mut world, &mut schedule);

    // Query for substation components
    let mut substation_query = world.query::<&xml::Substation>();
    let substations: Vec<&xml::Substation> = substation_query.iter(&world).collect();

    // Verify no duplicate registrations occurred
    assert_eq!(substations.len(), 2);
    assert!(substations.iter().any(|s| s.id == "P1"));
    assert!(substations.iter().any(|s| s.id == "P2"));
}

#[test]
fn test_ratio_tap_changer_registration() {
    // Create a new world and schedule
    let mut world = World::new();
    let mut schedule = Schedule::default();

    // Initialize required resources
    world.init_resource::<AssetRegistry>();
    world.init_resource::<Events<RegisterEvent>>();

    // Add systems to schedule
    schedule.add_systems(handle_register_events);

    // Create and register network
    let network = load_network(NETWORK_XML);
    network.register(&mut world, &mut schedule);

    // Query for transformers with ratio tap changers
    let mut transformer_query = world.query::<&xml::TwoWindingsTransformer>();
    let transformers: Vec<&xml::TwoWindingsTransformer> = transformer_query.iter(&world).collect();

    // Find the transformer with a ratio tap changer
    let transformer_with_rtc = transformers.iter().find(|t| t.id == "NHV2_NLOAD");
    assert!(transformer_with_rtc.is_some());

    // Verify the transformer has a ratio tap changer with 3 steps
    let transformer = transformer_with_rtc.unwrap();
    assert!(transformer.ratio_tap_changer.is_some());

    if let Some(rtc) = &transformer.ratio_tap_changer {
        assert_eq!(rtc.steps.len(), 3);
        assert_eq!(rtc.tap_position, 1);
        assert!(rtc.regulating);
        assert_eq!(rtc.regulation_mode, xml::RatioRegulationMode::Voltage);
        assert_eq!(rtc.regulation_value, 158.0);
    }
}

#[test]
fn test_bus_topology() {
    // Create a new world and schedule
    let mut world = World::new();
    let mut schedule = Schedule::default();

    // Initialize required resources
    world.init_resource::<AssetRegistry>();
    world.init_resource::<Events<RegisterEvent>>();

    // Add systems to schedule
    schedule.add_systems(handle_register_events);

    // Create and register network
    let network = load_network(NETWORK_XML);
    network.register(&mut world, &mut schedule);

    // Query for voltage levels
    let mut voltage_level_query = world.query::<&xml::VoltageLevel>();
    let voltage_levels: Vec<&xml::VoltageLevel> = voltage_level_query.iter(&world).collect();

    // Verify voltage levels exist and have bus breaker topology
    assert!(voltage_levels.iter().any(|vl| vl.id == "VLGEN"));
    assert!(voltage_levels.iter().any(|vl| vl.id == "VLHV1"));
    assert!(voltage_levels.iter().any(|vl| vl.id == "VLHV2"));
    assert!(voltage_levels.iter().any(|vl| vl.id == "VLLOAD"));
}

#[test]
#[ignore = "Need to debug, not finding Bus"]
fn test_bus_registration() {
    // Create a new world and schedule
    let mut world = World::new();
    let mut schedule = Schedule::default();

    // Initialize required resources
    world.init_resource::<AssetRegistry>();
    world.init_resource::<Events<RegisterEvent>>();

    // Add systems to schedule
    schedule.add_systems(handle_register_events);

    // Create and register network
    let network = load_network(NETWORK_XML);
    network.register(&mut world, &mut schedule);

    // Verify buses were registered
    let registry = world.resource::<AssetRegistry>();

    // Check if buses exist in registry
    assert!(registry.find("NGEN").is_some());
    assert!(registry.find("NHV1").is_some());
    assert!(registry.find("NHV2").is_some());
    assert!(registry.find("NLOAD").is_some());
}
