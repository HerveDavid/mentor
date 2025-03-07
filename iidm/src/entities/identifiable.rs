use enum_dispatch::enum_dispatch;

use super::*;

#[enum_dispatch(Identifiable)]
#[derive(Debug, Serialize, Deserialize)]
pub enum Identifiables {
    Network,
    Line,
    Substation,
    VoltageLevel,
    Generator,
    Load,
    Bus,
    BusbarSection,
    TwoWindingsTransformer,
    ThreeWindingsTransformer,
    Switch,
    ShuntCompensator,
    StaticVarCompensator,
    DanglingLine,
    TieLine,
    HvdcLine,
    HvdcConverterStation,
    TerminalRef,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coucou() {
        let n = Identifiables::from(Network::default());
        let i = n.id();
        assert_eq!(i, "");
    }
}
