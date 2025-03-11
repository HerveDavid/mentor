use bevy_ecs::prelude::*;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

use super::identifiable::Identifiables;
use super::resources::AssetRegistry;
use super::xml::*;

pub use iidm_derive::{Identifiable, Updatable};

#[enum_dispatch]
pub trait Identifiable: Clone + Component {
    fn id(&self) -> String;
    fn register(&self, world: &mut World, schedule: &mut Schedule);
    fn register_system(&self, commands: &mut Commands, registery: &mut ResMut<AssetRegistry>);
}

pub trait Updatable: Sized + Serialize + for<'de> Deserialize<'de> {
    type Updater: Send + Sync;

    fn update(&mut self, updates: Self::Updater);
}

pub trait JsonSchema: for<'de> Deserialize<'de> + Serialize {
    type Err;
    fn fields_json() -> Vec<String>;
    fn validate_json(json: &str) -> Result<Self, Self::Err>;
}
