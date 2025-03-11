use bevy_ecs::{event::Events, schedule::Schedule, world::World};
use iidm::*;

use super::*;

const HEAVY_NETWORK_XML: &str = "tests/data/PtFige-20250212-1455-enrichi.xiidm";

pub fn load_heavy_network() -> iidm::xml::Network {
    let test_network = std::fs::read_to_string(HEAVY_NETWORK_XML).unwrap();
    quick_xml::de::from_str(&test_network).unwrap()
}

#[test]
fn test_large_network() {
    // Create a new world and schedule
    let mut world = World::new();
    let mut schedule = Schedule::default();

    // Initialize required resources
    world.init_resource::<AssetRegistry>();
    world.init_resource::<Events<RegisterEvent<iidm::xml::Network>>>();

    // Add systems to schedule
    schedule.add_systems(handle_register_events::<iidm::xml::Network>);

    let network = load_heavy_network();
    network.register(&mut world, &mut schedule);
}

// #[test]
// fn test_large_network_register() {
//     // Create a new world and schedule
//     let mut world = World::new();
//     let mut schedule = Schedule::default();

//     // Initialize required resources
//     world.init_resource::<AssetRegistry>();
//     world.init_resource::<Events<RegisterEvent<iidm::xml::Network>>>();
//     world.init_resource::<Events<RegisterEvent<iidm::xml::Substation>>>();
//     world.init_resource::<Events<RegisterEvent<iidm::xml::VoltageLevel>>>();

//     // Add systems to schedule
//     schedule.add_systems(handle_register_events::<iidm::xml::Network>);
//     schedule.add_systems(handle_register_events::<iidm::xml::Substation>);
//     schedule.add_systems(handle_register_events::<iidm::xml::VoltageLevel>);

//     // Create and register network
//     let network = load_heavy_network();
//     network.register(&mut world, &mut schedule);

//     // Verify substations were registered
//     let registry = world.resource::<AssetRegistry>();

//     // Check if substations exist in registry
//     // assert!(registry.find("test_network").is_some());
//     // assert!(registry.find("sub1").is_some());
//     // assert!(registry.find("sub2").is_some());

//     // // Query for actual substation components
//     // let mut substation_query = world.query::<&Substation>();
//     // let substations: Vec<&Substation> = substation_query.iter(&world).collect();

//     // assert_eq!(substations.len(), 2);
//     // assert!(substations.iter().any(|s| s.id == "sub1"));
//     // assert!(substations.iter().any(|s| s.id == "sub2"));
// }

// #[test]
// fn test_network_register() {
//     // Create a new world and schedule
//     let mut world = World::new();
//     let mut schedule = Schedule::default();

//     // Initialize required resources
//     world.init_resource::<AssetRegistry>();
//     world.init_resource::<Events<RegisterEvent<Network>>>();
//     world.init_resource::<Events<RegisterEvent<Substation>>>();

//     // Add systems to schedule
//     schedule.add_systems(handle_register_events::<Network>);
//     schedule.add_systems(handle_register_events::<Substation>);

//     // Create and register network
//     let network = create_test_network();
//     network.register(&mut world, &mut schedule);

//     // Verify substations were registered
//     let registry = world.resource::<AssetRegistry>();

//     // Check if substations exist in registry
//     assert!(registry.find("test_network").is_some());
//     assert!(registry.find("sub1").is_some());
//     assert!(registry.find("sub2").is_some());

//     // Query for actual substation components
//     let mut substation_query = world.query::<&Substation>();
//     let substations: Vec<&Substation> = substation_query.iter(&world).collect();

//     assert_eq!(substations.len(), 2);
//     assert!(substations.iter().any(|s| s.id == "sub1"));
//     assert!(substations.iter().any(|s| s.id == "sub2"));
// }

// #[test]
// fn test_network_register_empty() {
//     let mut world = World::new();
//     let mut schedule = Schedule::default();

//     world.init_resource::<AssetRegistry>();
//     world.init_resource::<Events<RegisterEvent<Network>>>();
//     world.init_resource::<Events<RegisterEvent<Substation>>>();
//     schedule.add_systems(handle_register_events::<Network>);
//     schedule.add_systems(handle_register_events::<Substation>);

//     let mut network = create_test_network();
//     network.substations.clear();

//     network.register(&mut world, &mut schedule);

//     let mut substation_query = world.query::<&Substation>();
//     let substations: Vec<&Substation> = substation_query.iter(&world).collect();

//     assert_eq!(substations.len(), 0);
// }

// #[test]
// fn test_network_register_idempotency() {
//     let mut world = World::new();
//     let mut schedule = Schedule::default();

//     world.init_resource::<AssetRegistry>();
//     world.init_resource::<Events<RegisterEvent<Network>>>();
//     world.init_resource::<Events<RegisterEvent<Substation>>>();
//     schedule.add_systems(handle_register_events::<Network>);
//     schedule.add_systems(handle_register_events::<Substation>);

//     let network = create_test_network();

//     // Register twice
//     network.register(&mut world, &mut schedule);
//     network.register(&mut world, &mut schedule);

//     // Verify no duplicate registrations
//     let mut substation_query = world.query::<&Substation>();
//     let substations: Vec<&Substation> = substation_query.iter(&world).collect();

//     assert_eq!(substations.len(), 2);
//     assert!(substations.iter().any(|s| s.id == "sub1"));
//     assert!(substations.iter().any(|s| s.id == "sub2"));
// }
