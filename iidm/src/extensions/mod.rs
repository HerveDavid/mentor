use enum_dispatch::enum_dispatch;
pub use iidm_derive::{Identifiable, Updatable};

use bevy_ecs::{schedule::Schedule, world::World};
use serde::{Deserialize, Serialize};

use crate::entities::identifiable::Identifiables;
use crate::entities::updatable::{Updatables, Updaters};
use crate::entities::*;

#[enum_dispatch]
pub trait Identifiable {
    fn id(&self) -> String;
    fn register(&self, world: &mut World, schedule: &mut Schedule);
}

#[enum_dispatch]
pub trait UpdatableExt {
    fn update_ext(&mut self, updater: Updaters);
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
