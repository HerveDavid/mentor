use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

const NETWORK_FILE: &str = "tests/data/network.xiidm";

// Définition des structures
#[derive(Debug, Serialize, Deserialize)]
struct Root {
    #[serde(rename = "xmlns:iidm")]
    xmlns_p: Option<String>,

    #[serde(rename = "iidm:network")]
    element: Network,
}

#[derive(Debug, Serialize, Deserialize)]
struct Network {
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
}

#[derive(Debug, Serialize, Deserialize)]
struct Substation {
    #[serde(rename = "@id")]
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Line {
    #[serde(rename = "@id")]
    pub id: String,
}

#[test]
fn test_deserialize_from_xml() -> Result<(), Box<dyn std::error::Error>> {
    let test_network = std::fs::read_to_string(NETWORK_FILE)?;
    let network: Network = quick_xml::de::from_str(&test_network)?;

    assert_eq!(network.id, "sim1");
    assert_eq!(network.forecast_distance, 0);
    assert_eq!(network.source_format, "test");
    assert_eq!(network.minimum_validation_level, "STEADY_STATE_HYPOTHESIS");

    assert_eq!(network.substations.len(), 2);
    assert_eq!(network.lines.len(), 2);

    Ok(())
}
