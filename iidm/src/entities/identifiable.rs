use bevy_ecs::component::Component;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

use super::xml::*;

#[enum_dispatch(Identifiable)]
#[derive(Debug, Clone, Serialize, Deserialize, Component)]
pub enum Identifiables {
    Network,
    Substation,
    VoltageLevel,
    Generator,
    Load,
    BusbarSection,
    TwoWindingsTransformer,
    ThreeWindingsTransformer,
    Line,
    Switch,
    ShuntCompensator,
    StaticVarCompensator,
    DanglingLine,
    TieLine,
    HvdcLine,
    HvdcConverterStation,
    Battery,
    SeriesCompensator,
    LccConverterStation,
    VscConverterStation,
    Filter,
    GeographicalRegion,
    SubGeographicalRegion,
    SubstationGroup,
    SubstationRef,
    LineGroup,
    LineRef,
    HvdcLineGroup,
    HvdcLineRef,
    Contingency,
    ContingencyElement,
    TopologicalNode,
    TerminalRef,
    Bus,
}
