use chrono::{DateTime, FixedOffset};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Network {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@caseDate")]
    pub case_date: DateTime<FixedOffset>,

    #[serde(rename = "@forecastDistance")]
    pub forecast_distance: i32,

    #[serde(rename = "@sourceFormat")]
    pub source_format: String,

    #[serde(rename = "@minimumValidationLevel")]
    pub minimum_validation_level: String,

    #[serde(rename = "substation", default)]
    pub substations: Vec<Substation>,

    #[serde(rename = "line", default)]
    pub lines: Vec<Line>,

    #[serde(rename = "threeWindingsTransformer", default)]
    pub three_windings_transformers: Vec<ThreeWindingsTransformer>,

    #[serde(rename = "switch", default)]
    pub switches: Vec<Switch>,

    #[serde(rename = "shuntCompensator", default)]
    pub shunt_compensators: Vec<ShuntCompensator>,

    #[serde(rename = "staticVarCompensator", default)]
    pub static_var_compensators: Vec<StaticVarCompensator>,

    #[serde(rename = "danglingLine", default)]
    pub dangling_lines: Vec<DanglingLine>,

    #[serde(rename = "tieLine", default)]
    pub tie_lines: Vec<TieLine>,

    #[serde(rename = "hvdcLine", default)]
    pub hvdc_lines: Vec<HvdcLine>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Substation {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@country")]
    pub country: String,

    #[serde(rename = "@tso")]
    pub tso: String,

    #[serde(rename = "@geographicalTags")]
    pub geographical_tags: Vec<String>,

    #[serde(rename = "voltageLevel", default)]
    pub voltage_levels: Vec<VoltageLevel>,

    #[serde(rename = "twoWindingsTransformer", default)]
    pub two_windings_transformers: Vec<TwoWindingsTransformer>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoltageLevel {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@nominalV")]
    pub nominal_v: f64,

    #[serde(rename = "@topologyKind")]
    pub topology_kind: TopologyKind,

    #[serde(rename = "busBreakerTopology")]
    pub bus_breaker_topology: Option<BusBreakerTopology>,

    #[serde(rename = "nodeBreakerTopology")]
    pub node_breaker_topology: Option<NodeBreakerTopology>,

    #[serde(rename = "generator", default)]
    pub generators: Vec<Generator>,

    #[serde(rename = "load", default)]
    pub loads: Vec<Load>,

    #[serde(rename = "busbarSection", default)]
    pub busbar_sections: Vec<BusbarSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Display)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TopologyKind {
    NodeBreaker,
    BusBreaker,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Display)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EnergySource {
    Hydro,
    Nuclear,
    Wind,
    Thermal,
    Solar,
    Other,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Generator {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@energySource")]
    pub energy_source: EnergySource,
    #[serde(rename = "@minP")]
    pub min_p: f64,
    #[serde(rename = "@maxP")]
    pub max_p: f64,
    #[serde(rename = "@voltageRegulatorOn")]
    pub voltage_regulator_on: bool,
    #[serde(rename = "@targetP")]
    pub target_p: f64,
    #[serde(rename = "@targetV")]
    pub target_v: f64,
    #[serde(rename = "@targetQ")]
    pub target_q: f64,
    #[serde(rename = "@bus")]
    pub bus: String,
    #[serde(rename = "@connectableBus")]
    pub connectable_bus: String,

    #[serde(rename = "reactiveCapabilityCurve")]
    pub reactive_capability_curve: Option<ReactiveCapabilityCurve>,

    #[serde(rename = "minMaxReactiveLimits")]
    pub min_max_reactive_limits: Option<MinMaxReactiveLimits>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReactiveCapabilityCurve {
    #[serde(rename = "point")]
    pub points: Vec<ReactiveCapabilityCurvePoint>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReactiveCapabilityCurvePoint {
    #[serde(rename = "@p")]
    pub p: f64,
    #[serde(rename = "@minQ")]
    pub min_q: f64,
    #[serde(rename = "@maxQ")]
    pub max_q: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinMaxReactiveLimits {
    #[serde(rename = "@minQ")]
    pub min_q: f64,
    #[serde(rename = "@maxQ")]
    pub max_q: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Load {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@loadType")]
    pub load_type: LoadType,
    #[serde(rename = "@p0")]
    pub p0: f64,
    #[serde(rename = "@q0")]
    pub q0: f64,
    #[serde(rename = "@bus")]
    pub bus: String,
    #[serde(rename = "@connectableBus")]
    pub connectable_bus: String,

    #[serde(rename = "exponentialModel")]
    pub exponential_model: Option<ExponentialLoadModel>,

    #[serde(rename = "zipModel")]
    pub zip_model: Option<ZipLoadModel>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Display)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LoadType {
    Undefined,
    Auxiliary,
    FICTITIOUS,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExponentialLoadModel {
    #[serde(rename = "@p0")]
    pub p0: f64,
    #[serde(rename = "@q0")]
    pub q0: f64,
    #[serde(rename = "@np")]
    pub np: f64,
    #[serde(rename = "@nq")]
    pub nq: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZipLoadModel {
    #[serde(rename = "@p0")]
    pub p0: f64,
    #[serde(rename = "@q0")]
    pub q0: f64,
    #[serde(rename = "@zP")]
    pub z_p: f64,
    #[serde(rename = "@zQ")]
    pub z_q: f64,
    #[serde(rename = "@iP")]
    pub i_p: f64,
    #[serde(rename = "@iQ")]
    pub i_q: f64,
    #[serde(rename = "@pP")]
    pub p_p: f64,
    #[serde(rename = "@pQ")]
    pub p_q: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BusBreakerTopology {
    #[serde(rename = "bus", default)]
    pub buses: Vec<Bus>,

    #[serde(rename = "switch", default)]
    pub switches: Vec<Switch>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeBreakerTopology {
    #[serde(rename = "node")]
    pub nodes: Vec<Node>,

    #[serde(rename = "switch")]
    pub switches: Vec<Switch>,

    #[serde(rename = "internalConnection")]
    pub internal_connections: Vec<InternalConnection>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    #[serde(rename = "@id")]
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InternalConnection {
    #[serde(rename = "@node1")]
    pub node1: i32,
    #[serde(rename = "@node2")]
    pub node2: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bus {
    #[serde(rename = "@id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BusbarSection {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@bus")]
    pub bus: String,
    #[serde(rename = "@connectableBus")]
    pub connectable_bus: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwoWindingsTransformer {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@r")]
    pub r: f64,
    #[serde(rename = "@x")]
    pub x: f64,
    #[serde(rename = "@g")]
    pub g: f64,
    #[serde(rename = "@b")]
    pub b: f64,
    #[serde(rename = "@ratedU1")]
    pub rated_u1: f64,
    #[serde(rename = "@ratedU2")]
    pub rated_u2: f64,
    #[serde(rename = "@voltageLevelId1")]
    pub voltage_level_id1: String,
    #[serde(rename = "@bus1")]
    pub bus1: String,
    #[serde(rename = "@connectableBus1")]
    pub connectable_bus1: String,
    #[serde(rename = "@voltageLevelId2")]
    pub voltage_level_id2: String,
    #[serde(rename = "@bus2")]
    pub bus2: String,
    #[serde(rename = "@connectableBus2")]
    pub connectable_bus2: String,

    #[serde(rename = "ratioTapChanger")]
    pub ratio_tap_changer: Option<RatioTapChanger>,

    #[serde(rename = "phaseTapChanger")]
    pub phase_tap_changer: Option<PhaseTapChanger>,

    #[serde(rename = "currentLimits1")]
    pub current_limits1: Option<CurrentLimits>,

    #[serde(rename = "currentLimits2")]
    pub current_limits2: Option<CurrentLimits>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreeWindingsTransformer {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@r1")]
    pub r1: f64,
    #[serde(rename = "@x1")]
    pub x1: f64,
    #[serde(rename = "@g1")]
    pub g1: f64,
    #[serde(rename = "@b1")]
    pub b1: f64,
    #[serde(rename = "@ratedU1")]
    pub rated_u1: f64,
    #[serde(rename = "@r2")]
    pub r2: f64,
    #[serde(rename = "@x2")]
    pub x2: f64,
    #[serde(rename = "@g2")]
    pub g2: f64,
    #[serde(rename = "@b2")]
    pub b2: f64,
    #[serde(rename = "@ratedU2")]
    pub rated_u2: f64,
    #[serde(rename = "@r3")]
    pub r3: f64,
    #[serde(rename = "@x3")]
    pub x3: f64,
    #[serde(rename = "@g3")]
    pub g3: f64,
    #[serde(rename = "@b3")]
    pub b3: f64,
    #[serde(rename = "@ratedU3")]
    pub rated_u3: f64,
    #[serde(rename = "@voltageLevelId1")]
    pub voltage_level_id1: String,
    #[serde(rename = "@voltageLevelId2")]
    pub voltage_level_id2: String,
    #[serde(rename = "@voltageLevelId3")]
    pub voltage_level_id3: String,
    #[serde(rename = "@bus1")]
    pub bus1: String,
    #[serde(rename = "@bus2")]
    pub bus2: String,
    #[serde(rename = "@bus3")]
    pub bus3: String,
    #[serde(rename = "@connectableBus1")]
    pub connectable_bus1: String,
    #[serde(rename = "@connectableBus2")]
    pub connectable_bus2: String,
    #[serde(rename = "@connectableBus3")]
    pub connectable_bus3: String,

    #[serde(rename = "ratioTapChanger2")]
    pub ratio_tap_changer2: Option<RatioTapChanger>,

    #[serde(rename = "ratioTapChanger3")]
    pub ratio_tap_changer3: Option<RatioTapChanger>,

    #[serde(rename = "currentLimits1")]
    pub current_limits1: Option<CurrentLimits>,

    #[serde(rename = "currentLimits2")]
    pub current_limits2: Option<CurrentLimits>,

    #[serde(rename = "currentLimits3")]
    pub current_limits3: Option<CurrentLimits>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RatioTapChanger {
    #[serde(rename = "@regulating")]
    pub regulating: bool,
    #[serde(rename = "@lowTapPosition")]
    pub low_tap_position: i32,
    #[serde(rename = "@tapPosition")]
    pub tap_position: i32,
    #[serde(rename = "@targetDeadband")]
    pub target_deadband: f64,
    #[serde(rename = "@loadTapChangingCapabilities")]
    pub load_tap_changing_capabilities: bool,
    #[serde(rename = "@regulationMode")]
    pub regulation_mode: RatioRegulationMode,
    #[serde(rename = "@regulationValue")]
    pub regulation_value: f64,

    #[serde(rename = "terminalRef")]
    pub terminal_ref: TerminalRef,

    #[serde(rename = "step")]
    pub steps: Vec<TapStep>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhaseTapChanger {
    #[serde(rename = "@regulating")]
    pub regulating: bool,
    #[serde(rename = "@lowTapPosition")]
    pub low_tap_position: i32,
    #[serde(rename = "@tapPosition")]
    pub tap_position: i32,
    #[serde(rename = "@regulationMode")]
    pub regulation_mode: PhaseRegulationMode,
    #[serde(rename = "@regulationValue")]
    pub regulation_value: f64,
    #[serde(rename = "@targetDeadband")]
    pub target_deadband: f64,

    #[serde(rename = "terminalRef")]
    pub terminal_ref: TerminalRef,

    #[serde(rename = "step")]
    pub steps: Vec<PhaseTapStep>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PhaseRegulationMode {
    CURRENT_LIMITER,
    ACTIVE_POWER_CONTROL,
    FIXED_TAP,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RatioRegulationMode {
    VOLTAGE,
    REACTIVE_POWER,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Line {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@r")]
    pub r: f64,
    #[serde(rename = "@x")]
    pub x: f64,
    #[serde(rename = "@b1")]
    pub b1: f64,
    #[serde(rename = "@b2")]
    pub b2: f64,
    #[serde(rename = "@g1")]
    pub g1: f64,
    #[serde(rename = "@g2")]
    pub g2: f64,
    #[serde(rename = "@voltageLevelId1")]
    pub voltage_level_id1: String,
    #[serde(rename = "@bus1")]
    pub bus1: String,
    #[serde(rename = "@connectableBus1")]
    pub connectable_bus1: String,
    #[serde(rename = "@voltageLevelId2")]
    pub voltage_level_id2: String,
    #[serde(rename = "@bus2")]
    pub bus2: String,
    #[serde(rename = "@connectableBus2")]
    pub connectable_bus2: String,

    #[serde(rename = "currentLimits1")]
    pub current_limits1: Option<CurrentLimits>,

    #[serde(rename = "currentLimits2")]
    pub current_limits2: Option<CurrentLimits>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Switch {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@kind")]
    pub kind: SwitchKind,
    #[serde(rename = "@open")]
    pub open: bool,
    #[serde(rename = "@retained")]
    pub retained: bool,
    #[serde(rename = "@bus1")]
    pub bus1: String,
    #[serde(rename = "@bus2")]
    pub bus2: String,
    #[serde(rename = "@voltageLevelId")]
    pub voltage_level_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SwitchKind {
    BREAKER,
    DISCONNECTOR,
    LOAD_BREAK_SWITCH,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShuntCompensator {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@bPerSection")]
    pub b_per_section: f64,
    #[serde(rename = "@maximumSectionCount")]
    pub maximum_section_count: i32,
    #[serde(rename = "@sectionCount")]
    pub section_count: i32,
    #[serde(rename = "@bus")]
    pub bus: String,
    #[serde(rename = "@connectableBus")]
    pub connectable_bus: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StaticVarCompensator {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@bMin")]
    pub b_min: f64,
    #[serde(rename = "@bMax")]
    pub b_max: f64,
    #[serde(rename = "@regulationMode")]
    pub regulation_mode: StaticVarCompensatorRegulationMode,
    #[serde(rename = "@voltageSetpoint")]
    pub voltage_setpoint: f64,
    #[serde(rename = "@reactivePowerSetpoint")]
    pub reactive_power_setpoint: f64,
    #[serde(rename = "@bus")]
    pub bus: String,
    #[serde(rename = "@connectableBus")]
    pub connectable_bus: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StaticVarCompensatorRegulationMode {
    VOLTAGE,
    REACTIVE_POWER,
    OFF,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DanglingLine {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@p0")]
    pub p0: f64,
    #[serde(rename = "@q0")]
    pub q0: f64,
    #[serde(rename = "@r")]
    pub r: f64,
    #[serde(rename = "@x")]
    pub x: f64,
    #[serde(rename = "@g")]
    pub g: f64,
    #[serde(rename = "@b")]
    pub b: f64,
    #[serde(rename = "@bus")]
    pub bus: String,
    #[serde(rename = "@connectableBus")]
    pub connectable_bus: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TieLine {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "danglingLine1")]
    pub dangling_line1: DanglingLine,

    #[serde(rename = "danglingLine2")]
    pub dangling_line2: DanglingLine,

    #[serde(rename = "@ucteXnodeCode")]
    pub ucte_xnode_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HvdcLine {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@r")]
    pub resistance: f64,
    #[serde(rename = "@nominalV")]
    pub nominal_v: f64,
    #[serde(rename = "@convertersMode")]
    pub converters_mode: ConvertersMode,
    #[serde(rename = "@activePowerSetpoint")]
    pub active_power_setpoint: f64,
    #[serde(rename = "@maxP")]
    pub max_p: f64,

    #[serde(rename = "converterStation1")]
    pub converter_station1: HvdcConverterStation,

    #[serde(rename = "converterStation2")]
    pub converter_station2: HvdcConverterStation,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HvdcConverterStation {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@voltageLevelId")]
    pub voltage_level_id: String,
    #[serde(rename = "@bus")]
    pub bus: String,
    #[serde(rename = "@connectableBus")]
    pub connectable_bus: String,
    #[serde(rename = "@lossFactor")]
    pub loss_factor: f64,
    #[serde(rename = "@reactivePowerSetpoint")]
    pub reactive_power_setpoint: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConvertersMode {
    SIDE1_RECTIFIER_SIDE2_INVERTER,
    SIDE1_INVERTER_SIDE2_RECTIFIER,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TerminalRef {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@side")]
    pub side: Side,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TapStep {
    #[serde(rename = "@r")]
    pub r: f64,
    #[serde(rename = "@x")]
    pub x: f64,
    #[serde(rename = "@g")]
    pub g: f64,
    #[serde(rename = "@b")]
    pub b: f64,
    #[serde(rename = "@rho")]
    pub rho: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhaseTapStep {
    #[serde(rename = "@r")]
    pub r: f64,
    #[serde(rename = "@x")]
    pub x: f64,
    #[serde(rename = "@g")]
    pub g: f64,
    #[serde(rename = "@b")]
    pub b: f64,
    #[serde(rename = "@rho")]
    pub rho: f64,
    #[serde(rename = "@alpha")]
    pub alpha: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentLimits {
    #[serde(rename = "@permanentLimit")]
    pub permanent_limit: f64,

    #[serde(rename = "temporaryLimit")]
    pub temporary_limits: Vec<TemporaryLimit>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemporaryLimit {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@acceptableDuration")]
    pub acceptable_duration: i32,
    #[serde(rename = "@value")]
    pub value: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Side {
    ONE,
    TWO,
    THREE,
}
