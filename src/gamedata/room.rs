pub use super::address::Address;

use anyhow::{anyhow, Context, Result};
use serde::Deserialize;
use std::{ffi::OsStr, fs, path::Path};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Room {
    pub id: u16,
    pub name: String,
    pub area: String,
    pub subarea: String,
    pub playable: bool,
    pub room_address: Address,
    pub nodes: Vec<RoomNode>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomNode {
    pub id: u16,
    pub name: String,
    pub node_type: String,
    pub node_sub_type: String,
    #[serde(default)]
    pub node_address: Option<Address>,
    #[serde(default)]
    pub door_orientation: Option<String>,
}

pub fn load_all_rooms(sm_json_data_path: impl AsRef<Path>) -> Result<Vec<Room>> {
    let regions_dir = sm_json_data_path.as_ref().join("region");

    let mut rooms: Vec<Room> = vec![];
    let files = fs::read_dir(regions_dir).context("opening sm-json-data root dir")?;
    for file in files {
        let file = file.context("reading entry")?;
        let meta = file.metadata().context("getting file metadata")?;
        if meta.is_dir() {
            let path = file.path();
            load_region(&mut rooms, &path)
                .with_context(|| format!("loading rooms from: {:?}", &path))?;
        }
    }
    Ok(rooms)
}

fn load_region(rooms: &mut Vec<Room>, regions_dir: &Path) -> Result<()> {
    let files = fs::read_dir(regions_dir).context("opening region dir")?;
    for file in files {
        let file = file.context("reading entry")?;
        let path = file.path();
        if path.is_dir() {
            load_rooms(rooms, &path).context("loading room group dir")?;
        }
    }
    Ok(())
}

fn load_rooms(rooms: &mut Vec<Room>, room_group_dir: &Path) -> Result<()> {
    let files = fs::read_dir(room_group_dir).context("opening room dir")?;
    for file in files {
        let file = file.context("reading entry")?;
        let path = file.path();
        if path.extension().and_then(OsStr::to_str) == Some("json") {
            let room = load_room_json(&path)
                .with_context(|| anyhow!("loading room from file: {:?}", path))?;
            rooms.push(room);
        }
    }
    Ok(())
}

fn load_room_json(path: &Path) -> Result<Room> {
    let file = fs::File::open(path).context("opening room file")?;
    serde_json::from_reader(file).context("parsing room JSON")
}
