use std::{collections::HashMap, fs, path::Path};

use anyhow::{anyhow, Context, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RegionDiagram {
    #[serde(default)]
    pub origin: (u8, u8),

    #[serde(flatten)]
    pub rooms: HashMap<String, RoomDiagram>,
}

#[derive(Debug, Deserialize)]
pub struct RoomDiagram {
    pub origin: (u8, u8),
    #[serde(default = "one")]
    pub width: u8,
    #[serde(default = "one")]
    pub height: u8,
}

pub fn load_room_diagrms(
    sm_json_data_path: impl AsRef<Path>,
) -> Result<HashMap<String, RegionDiagram>> {
    let region_dir = sm_json_data_path.as_ref().join("region");

    let mut diagrams: HashMap<String, RegionDiagram> = HashMap::new();
    let files = fs::read_dir(&region_dir).context("opening sm-json-data root dir")?;
    for file in files {
        let file = file.context("reading entry")?;
        let meta = file.metadata().context("getting file metadata")?;
        let region_name = file
            .file_name()
            .to_str()
            .ok_or_else(|| anyhow!("invalid UTF8 filename: {:?}", &region_dir))?
            .to_owned();
        if meta.is_dir() {
            let path = file.path();

            let diagram = load_diagram_json(&path.join("roomDiagrams.json"))
                .with_context(|| format!("loading room diagrams from: {:?}", &path))?;
            diagrams.insert(region_name, diagram);
        }
    }
    Ok(diagrams)
}

fn load_diagram_json(path: &Path) -> Result<RegionDiagram> {
    let file = fs::File::open(path).context("opening room diagram file")?;
    serde_json::from_reader(file).context("parsing json")
}

fn one() -> u8 {
    1
}
