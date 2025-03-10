use chrono::{DateTime, FixedOffset};
use derive_more::Display;
use serde::{Deserialize, Serialize};

/// Structure racine du réseau
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

/// Structure représentant une sous-station
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Substation {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@country")]
    pub country: String,

    #[serde(rename = "@tso", default)]
    pub tso: String,

    #[serde(rename = "@geographicalTags", default)]
    pub geographical_tags: Vec<String>,

    #[serde(rename = "voltageLevel", default)]
    pub voltage_levels: Vec<VoltageLevel>,

    #[serde(rename = "twoWindingsTransformer", default)]
    pub two_windings_transformers: Vec<TwoWindingsTransformer>,
}

/// Structure représentant un niveau de tension
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

/// Types de topologie
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Display)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TopologyKind {
    NodeBreaker,
    BusBreaker,
}

/// Types de source d'énergie
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

/// Structure représentant un générateur
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

    #[serde(rename = "@bus", default)]
    pub bus: Option<String>,

    #[serde(rename = "@connectableBus", default)]
    pub connectable_bus: Option<String>,

    #[serde(rename = "reactiveCapabilityCurve")]
    pub reactive_capability_curve: Option<ReactiveCapabilityCurve>,

    #[serde(rename = "minMaxReactiveLimits")]
    pub min_max_reactive_limits: Option<MinMaxReactiveLimits>,
}

/// Structure pour la courbe de capacité réactive
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReactiveCapabilityCurve {
    #[serde(rename = "point")]
    pub points: Vec<ReactiveCapabilityCurvePoint>,
}

/// Point de la courbe de capacité réactive
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

/// Structure pour les limites réactives min-max
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinMaxReactiveLimits {
    #[serde(rename = "@minQ")]
    pub min_q: f64,

    #[serde(rename = "@maxQ")]
    pub max_q: f64,
}

/// Structure représentant une charge
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

    #[serde(rename = "@bus", default)]
    pub bus: Option<String>,

    #[serde(rename = "@connectableBus", default)]
    pub connectable_bus: Option<String>,

    #[serde(rename = "exponentialModel")]
    pub exponential_model: Option<ExponentialLoadModel>,

    #[serde(rename = "zipModel")]
    pub zip_model: Option<ZipLoadModel>,
}

/// Types de charge
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Display)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LoadType {
    Undefined,
    Auxiliary,
    Fictitious,
}

/// Modèle exponentiel de charge
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

/// Modèle ZIP de charge
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

/// Structure pour la topologie bus-disjoncteur
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BusBreakerTopology {
    #[serde(rename = "bus", default)]
    pub buses: Vec<Bus>,

    #[serde(rename = "switch", default)]
    pub switches: Vec<Switch>,
}

/// Structure pour la topologie nœud-disjoncteur
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeBreakerTopology {
    #[serde(rename = "busbarSection", default)]
    pub busbar_sections: Vec<BusbarSection>,

    #[serde(rename = "bus", default)]
    pub buses: Vec<CalculatedBus>,

    #[serde(rename = "switch", default)]
    pub switches: Vec<Switch>,

    #[serde(rename = "internalConnection", default)]
    pub internal_connections: Vec<InternalConnection>,
}

/// Structure pour les bus calculés
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalculatedBus {
    #[serde(rename = "@angle")]
    pub angle: f64,

    #[serde(rename = "@nodes")]
    pub nodes: Vec<String>,

    #[serde(rename = "@v")]
    pub v: f64,
}

/// Structure pour les connexions internes
#[derive(Debug, Serialize, Deserialize)]
pub struct InternalConnection {
    #[serde(rename = "@node1")]
    pub node1: i32,

    #[serde(rename = "@node2")]
    pub node2: i32,
}

/// Structure pour un bus
#[derive(Debug, Serialize, Deserialize)]
pub struct Bus {
    #[serde(rename = "@id")]
    pub id: String,
}

/// Structure pour une section de jeu de barres
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BusbarSection {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@name", default)]
    pub name: String,

    #[serde(rename = "@node")]
    pub node: i32,
}

/// Structure pour un transformateur à deux enroulements
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

    #[serde(rename = "@bus1", default)]
    pub bus1: Option<String>,

    #[serde(rename = "@connectableBus1", default)]
    pub connectable_bus1: Option<String>,

    #[serde(rename = "@voltageLevelId2")]
    pub voltage_level_id2: String,

    #[serde(rename = "@bus2", default)]
    pub bus2: Option<String>,

    #[serde(rename = "@connectableBus2", default)]
    pub connectable_bus2: Option<String>,

    #[serde(rename = "ratioTapChanger")]
    pub ratio_tap_changer: Option<RatioTapChanger>,

    #[serde(rename = "phaseTapChanger")]
    pub phase_tap_changer: Option<PhaseTapChanger>,

    #[serde(rename = "currentLimits1")]
    pub current_limits1: Option<CurrentLimits>,

    #[serde(rename = "currentLimits2")]
    pub current_limits2: Option<CurrentLimits>,
}

/// Structure pour un transformateur à trois enroulements
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

    #[serde(rename = "@bus1", default)]
    pub bus1: Option<String>,

    #[serde(rename = "@bus2", default)]
    pub bus2: Option<String>,

    #[serde(rename = "@bus3", default)]
    pub bus3: Option<String>,

    #[serde(rename = "@connectableBus1", default)]
    pub connectable_bus1: Option<String>,

    #[serde(rename = "@connectableBus2", default)]
    pub connectable_bus2: Option<String>,

    #[serde(rename = "@connectableBus3", default)]
    pub connectable_bus3: Option<String>,

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

/// Structure pour un régleur en charge
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RatioTapChanger {
    #[serde(rename = "@regulating", default)]
    pub regulating: bool,

    #[serde(rename = "@lowTapPosition")]
    pub low_tap_position: i32,

    #[serde(rename = "@tapPosition")]
    pub tap_position: i32,

    #[serde(rename = "@targetDeadband", default)]
    pub target_deadband: Option<f64>,

    #[serde(rename = "@loadTapChangingCapabilities")]
    pub load_tap_changing_capabilities: bool,

    #[serde(rename = "@regulationMode", default)]
    pub regulation_mode: RatioRegulationMode,

    #[serde(rename = "@regulationValue", default)]
    pub regulation_value: f64,

    #[serde(rename = "terminalRef", default)]
    pub terminal_ref: Option<TerminalRef>,

    #[serde(rename = "step")]
    pub steps: Vec<TapStep>,
}

/// Structure pour un régleur en phase
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhaseTapChanger {
    #[serde(rename = "@regulating")]
    pub regulating: bool,

    #[serde(rename = "@lowTapPosition")]
    pub low_tap_position: i32,

    #[serde(rename = "@tapPosition")]
    pub tap_position: i32,

    #[serde(rename = "@regulationMode", default)]
    pub regulation_mode: PhaseRegulationMode,

    #[serde(rename = "@regulationValue", default)]
    pub regulation_value: f64,

    #[serde(rename = "@targetDeadband", default)]
    pub target_deadband: Option<f64>,

    #[serde(rename = "terminalRef", default)]
    pub terminal_ref: Option<TerminalRef>,

    #[serde(rename = "step")]
    pub steps: Vec<PhaseTapStep>,
}

/// Modes de régulation des régleurs en phase
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Display)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PhaseRegulationMode {
    CurrentLimiter,
    ActivePowerControl,
    FixedTap,
}

impl Default for PhaseRegulationMode {
    fn default() -> Self {
        Self::CurrentLimiter
    }
}

/// Modes de régulation des régleurs en charge
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Display)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RatioRegulationMode {
    Voltage,
    ReactivePower,
}

impl Default for RatioRegulationMode {
    fn default() -> Self {
        Self::Voltage
    }
}

/// Structure pour une ligne
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

    #[serde(rename = "@bus1", default)]
    pub bus1: Option<String>,

    #[serde(rename = "@connectableBus1", default)]
    pub connectable_bus1: Option<String>,

    #[serde(rename = "@voltageLevelId2")]
    pub voltage_level_id2: String,

    #[serde(rename = "@bus2", default)]
    pub bus2: Option<String>,

    #[serde(rename = "@connectableBus2", default)]
    pub connectable_bus2: Option<String>,

    #[serde(rename = "currentLimits1")]
    pub current_limits1: Option<CurrentLimits>,

    #[serde(rename = "currentLimits2")]
    pub current_limits2: Option<CurrentLimits>,
}

/// Structure pour un interrupteur
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

    #[serde(rename = "@bus1", default)]
    pub bus1: Option<String>,

    #[serde(rename = "@bus2", default)]
    pub bus2: Option<String>,

    #[serde(rename = "@voltageLevelId", default)]
    pub voltage_level_id: Option<String>,

    #[serde(rename = "@node1", default)]
    pub node1: Option<i32>,

    #[serde(rename = "@node2", default)]
    pub node2: Option<i32>,
}

/// Types d'interrupteurs
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Display)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SwitchKind {
    Breaker,
    Disconnector,
    LoadBreakSwitch,
}

/// Structure pour un compensateur shunt
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

    #[serde(rename = "@voltageLevelId")]
    pub voltage_level_id: String,

    #[serde(rename = "@bus", default)]
    pub bus: Option<String>,

    #[serde(rename = "@connectableBus")]
    pub connectable_bus: String,

    #[serde(rename = "@node", default)]
    pub node: Option<i32>,
}

/// Structure pour un compensateur statique de puissance réactive
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StaticVarCompensator {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@bMin")]
    pub b_min: f64,

    #[serde(rename = "@bMax")]
    pub b_max: f64,

    #[serde(rename = "@regulationMode", default)]
    pub regulation_mode: StaticVarCompensatorRegulationMode,

    #[serde(rename = "@voltageSetpoint")]
    pub voltage_setpoint: f64,

    #[serde(rename = "@reactivePowerSetpoint")]
    pub reactive_power_setpoint: f64,

    #[serde(rename = "@voltageLevelId")]
    pub voltage_level_id: String,

    #[serde(rename = "@bus", default)]
    pub bus: Option<String>,

    #[serde(rename = "@connectableBus")]
    pub connectable_bus: String,

    #[serde(rename = "@node", default)]
    pub node: Option<i32>,
}

/// Modes de régulation des compensateurs statiques
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Display)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StaticVarCompensatorRegulationMode {
    Voltage,
    ReactivePower,
    Off,
}

impl Default for StaticVarCompensatorRegulationMode {
    fn default() -> Self {
        Self::Off
    }
}

/// Structure pour une ligne pendante
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

    #[serde(rename = "@voltageLevelId")]
    pub voltage_level_id: String,

    #[serde(rename = "@bus", default)]
    pub bus: Option<String>,

    #[serde(rename = "@connectableBus")]
    pub connectable_bus: String,

    #[serde(rename = "@node", default)]
    pub node: Option<i32>,

    #[serde(rename = "currentLimits")]
    pub current_limits: Option<CurrentLimits>,
}

/// Structure pour une ligne d'attache
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TieLine {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@name", default)]
    pub name: String,

    #[serde(rename = "danglingLine1")]
    pub dangling_line1: DanglingLine,

    #[serde(rename = "danglingLine2")]
    pub dangling_line2: DanglingLine,

    #[serde(rename = "@ucteXnodeCode")]
    pub ucte_xnode_code: String,
}

/// Structure pour une ligne HVDC
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HvdcLine {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@name", default)]
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

    #[serde(rename = "@converterStation1")]
    pub converter_station1: String,

    #[serde(rename = "@converterStation2")]
    pub converter_station2: String,
}

/// Structure pour une station de conversion HVDC
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HvdcConverterStation {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@name", default)]
    pub name: String,

    #[serde(rename = "@voltageLevelId")]
    pub voltage_level_id: String,

    #[serde(rename = "@bus", default)]
    pub bus: Option<String>,

    #[serde(rename = "@connectableBus")]
    pub connectable_bus: String,

    #[serde(rename = "@node", default)]
    pub node: Option<i32>,

    #[serde(rename = "@lossFactor")]
    pub loss_factor: f64,

    #[serde(rename = "@reactivePowerSetpoint")]
    pub reactive_power_setpoint: f64,
}

/// Modes des convertisseurs HVDC
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Display)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ConvertersMode {
    #[serde(rename = "SIDE_1_SECTIFIER_SIDE_2_INVERTER")]
    Side1RectifierSide2Inverter,
    #[serde(rename = "SIDE_1_INVERTER_SIDE_2_RECTIFIER")]
    Side1InverterSide2Rectifier,
}

/// Structure pour référence à une borne
#[derive(Debug, Serialize, Deserialize)]
pub struct TerminalRef {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@side")]
    pub side: Side,
}

/// Structure pour une étape de régleur
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

/// Structure pour une étape de régleur de phase
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

/// Structure pour les limites de courant
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentLimits {
    #[serde(rename = "@permanentLimit")]
    pub permanent_limit: f64,

    #[serde(rename = "temporaryLimit", default)]
    pub temporary_limits: Vec<TemporaryLimit>,
}

/// Structure pour une limite temporaire
#[derive(Debug, Serialize, Deserialize)]
pub struct TemporaryLimit {
    #[serde(rename = "@name", default)]
    pub name: String,

    #[serde(rename = "@acceptableDuration")]
    pub acceptable_duration: i32,

    #[serde(rename = "@value")]
    pub value: f64,
}

/// Enum pour le côté (utilisé dans TerminalRef)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Display)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Side {
    One,
    Two,
    Three,
}

/// Structure pour une batterie
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Battery {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@minP")]
    pub min_p: f64,

    #[serde(rename = "@maxP")]
    pub max_p: f64,

    #[serde(rename = "@targetP")]
    pub target_p: f64,

    #[serde(rename = "@targetQ")]
    pub target_q: f64,

    #[serde(rename = "@voltageLevelId")]
    pub voltage_level_id: String,

    #[serde(rename = "@bus", default)]
    pub bus: Option<String>,

    #[serde(rename = "@connectableBus")]
    pub connectable_bus: String,

    #[serde(rename = "@node", default)]
    pub node: Option<i32>,

    #[serde(rename = "minMaxReactiveLimits")]
    pub min_max_reactive_limits: Option<MinMaxReactiveLimits>,
}

/// Structure pour une liaison série
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SeriesCompensator {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@r")]
    pub r: f64,

    #[serde(rename = "@x")]
    pub x: f64,

    #[serde(rename = "@voltageLevelId1")]
    pub voltage_level_id1: String,

    #[serde(rename = "@voltageLevelId2")]
    pub voltage_level_id2: String,

    #[serde(rename = "@bus1", default)]
    pub bus1: Option<String>,

    #[serde(rename = "@bus2", default)]
    pub bus2: Option<String>,

    #[serde(rename = "@connectableBus1")]
    pub connectable_bus1: String,

    #[serde(rename = "@connectableBus2")]
    pub connectable_bus2: String,

    #[serde(rename = "@node1", default)]
    pub node1: Option<i32>,

    #[serde(rename = "@node2", default)]
    pub node2: Option<i32>,

    #[serde(rename = "currentLimits")]
    pub current_limits: Option<CurrentLimits>,
}

/// Structure pour un LCC (Line Commutated Converter)
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LccConverterStation {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@name", default)]
    pub name: String,

    #[serde(rename = "@voltageLevelId")]
    pub voltage_level_id: String,

    #[serde(rename = "@bus", default)]
    pub bus: Option<String>,

    #[serde(rename = "@connectableBus")]
    pub connectable_bus: String,

    #[serde(rename = "@node", default)]
    pub node: Option<i32>,

    #[serde(rename = "@lossFactor")]
    pub loss_factor: f64,

    #[serde(rename = "@powerFactor")]
    pub power_factor: f64,
}

/// Structure pour un VSC (Voltage Source Converter)
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VscConverterStation {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@name", default)]
    pub name: String,

    #[serde(rename = "@voltageLevelId")]
    pub voltage_level_id: String,

    #[serde(rename = "@bus", default)]
    pub bus: Option<String>,

    #[serde(rename = "@connectableBus")]
    pub connectable_bus: String,

    #[serde(rename = "@node", default)]
    pub node: Option<i32>,

    #[serde(rename = "@lossFactor")]
    pub loss_factor: f64,

    #[serde(rename = "@voltageRegulatorOn")]
    pub voltage_regulator_on: bool,

    #[serde(rename = "@voltageSetpoint")]
    pub voltage_setpoint: f64,

    #[serde(rename = "@reactivePowerSetpoint")]
    pub reactive_power_setpoint: f64,

    #[serde(rename = "reactiveCapabilityCurve")]
    pub reactive_capability_curve: Option<ReactiveCapabilityCurve>,

    #[serde(rename = "minMaxReactiveLimits")]
    pub min_max_reactive_limits: Option<MinMaxReactiveLimits>,
}

/// Structure pour un poste à courant alternatif
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivePowerControl {
    #[serde(rename = "@participate")]
    pub participate: bool,

    #[serde(rename = "@droop")]
    pub droop: f64,
}

/// Structure pour représenter une extension d'attributs
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Extension {
    #[serde(rename = "$text")]
    pub content: String,
}

/// Structure pour un filtre AC
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Filter {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@voltageLevelId")]
    pub voltage_level_id: String,

    #[serde(rename = "@bus", default)]
    pub bus: Option<String>,

    #[serde(rename = "@connectableBus")]
    pub connectable_bus: String,

    #[serde(rename = "@node", default)]
    pub node: Option<i32>,

    #[serde(rename = "@bPerSection")]
    pub b_per_section: f64,

    #[serde(rename = "@maximumSectionCount")]
    pub maximum_section_count: i32,

    #[serde(rename = "@sectionCount")]
    pub section_count: i32,
}

/// Structure pour une zone géographique
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeographicalRegion {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@name", default)]
    pub name: String,

    #[serde(rename = "subRegion", default)]
    pub sub_regions: Vec<SubGeographicalRegion>,
}

/// Structure pour une sous-zone géographique
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubGeographicalRegion {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@name", default)]
    pub name: String,

    #[serde(rename = "@regionId")]
    pub region_id: String,
}

/// Structure pour les valeurs de tension
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoltageLevelData {
    #[serde(rename = "@nominalV")]
    pub nominal_v: f64,

    #[serde(rename = "@highVoltageLimit")]
    pub high_voltage_limit: f64,

    #[serde(rename = "@lowVoltageLimit")]
    pub low_voltage_limit: f64,
}

/// Structure pour un groupe de sous-stations
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstationGroup {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@name", default)]
    pub name: String,

    #[serde(rename = "substationRef", default)]
    pub substation_refs: Vec<SubstationRef>,
}

/// Structure pour une référence à une sous-station
#[derive(Debug, Serialize, Deserialize)]
pub struct SubstationRef {
    #[serde(rename = "@id")]
    pub id: String,
}

/// Structure pour un groupe de lignes
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LineGroup {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@name", default)]
    pub name: String,

    #[serde(rename = "lineRef", default)]
    pub line_refs: Vec<LineRef>,
}

/// Structure pour une référence à une ligne
#[derive(Debug, Serialize, Deserialize)]
pub struct LineRef {
    #[serde(rename = "@id")]
    pub id: String,
}

/// Structure pour un groupe de liaisons HVDC
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HvdcLineGroup {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@name", default)]
    pub name: String,

    #[serde(rename = "hvdcLineRef", default)]
    pub hvdc_line_refs: Vec<HvdcLineRef>,
}

/// Structure pour une référence à une liaison HVDC
#[derive(Debug, Serialize, Deserialize)]
pub struct HvdcLineRef {
    #[serde(rename = "@id")]
    pub id: String,
}

/// Structure pour une équation PSSE
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PsseModel {
    #[serde(rename = "@code")]
    pub code: i32,

    #[serde(rename = "@parameterCount")]
    pub parameter_count: i32,

    #[serde(rename = "parameter", default)]
    pub parameters: Vec<PsseParameter>,
}

/// Structure pour un paramètre PSSE
#[derive(Debug, Serialize, Deserialize)]
pub struct PsseParameter {
    #[serde(rename = "@name", default)]
    pub name: String,

    #[serde(rename = "@value")]
    pub value: f64,
}

/// Structure pour un contingent
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contingency {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@name", default)]
    pub name: String,

    #[serde(rename = "element", default)]
    pub elements: Vec<ContingencyElement>,
}

/// Structure pour un élément de contingent
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContingencyElement {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@type")]
    pub element_type: ContingencyElementType,
}

/// Types d'éléments de contingent
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Display)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ContingencyElementType {
    Branch,
    Line,
    Generator,
    BusbarSection,
    HvdcLine,
    StaticVarCompensator,
    DanglingLine,
}

/// Structure pour les méta-données
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    #[serde(rename = "@name", default)]
    pub name: String,

    #[serde(rename = "@value")]
    pub value: String,
}

/// Structure pour un nœud topologique
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopologicalNode {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@code")]
    pub code: String,

    #[serde(rename = "@voltageLevelId")]
    pub voltage_level_id: String,
}

/// Structure pour l'état initial
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitialState {
    #[serde(rename = "@voltageLevelId")]
    pub voltage_level_id: String,

    #[serde(rename = "@busId")]
    pub bus_id: String,

    #[serde(rename = "@v")]
    pub v: f64,

    #[serde(rename = "@angle")]
    pub angle: f64,
}

/// Structure pour les paramètres de calcul de flux de puissance
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadFlowParameters {
    #[serde(rename = "@voltageInitMode")]
    pub voltage_init_mode: VoltageInitMode,

    #[serde(rename = "@transformerVoltageControlOn")]
    pub transformer_voltage_control_on: bool,

    #[serde(rename = "@phaseShifterRegulationOn")]
    pub phase_shifter_regulation_on: bool,

    #[serde(rename = "@noGeneratorReactiveLimits")]
    pub no_generator_reactive_limits: bool,

    #[serde(rename = "@specificCompatibility")]
    pub specific_compatibility: bool,
}

/// Modes d'initialisation de tension
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Display)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VoltageInitMode {
    Dc,
    Uniform,
    Previous,
    Flat,
}

/// Types de paramètre de limites
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Display)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LimitType {
    Apparent,
    Active,
    Current,
    Voltage,
}

/// Structure pour les paramètres de sécurité N-1
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecurityAnalysisParameters {
    #[serde(rename = "@loadFlowParameters")]
    pub load_flow_parameters: Option<LoadFlowParameters>,

    #[serde(rename = "@limitReduction")]
    pub limit_reduction: f64,

    #[serde(rename = "@limitType")]
    pub limit_type: LimitType,
}
