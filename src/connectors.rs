use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ConnectorType {
    Stud,
    Tube,
    Pin,
    AxleHole,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub enum Direction {
    #[serde(rename = "+x")]
    PosX,
    #[serde(rename = "-x")]
    NegX,
    #[serde(rename = "+y")]
    PosY,
    #[serde(rename = "-y")]
    NegY,
    #[serde(rename = "+z")]
    PosZ,
    #[serde(rename = "-z")]
    NegZ,
}

#[derive(Debug, Deserialize)]
pub struct ConnectorFile {
    pub connectors: Vec<Connector>,
}

#[derive(Debug, Deserialize)]
pub struct Connector {
    #[serde(rename = "type")]
    pub kind: ConnectorType,
    pub pos: Position,
    pub dir: Direction,
}

#[derive(Debug, Deserialize)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl ConnectorFile {
    pub fn validate(&self) -> Result<(), String> {
        // If it deserializes, type and direction are already valid.
        Ok(())
    }
}
