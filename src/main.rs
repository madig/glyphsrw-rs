use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::Read;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Font {
    #[serde(rename = ".appVersion")]
    app_version: String,
    copyright: String,
    custom_parameters: Vec<HashMap<String, serde_json::Value>>,
    date: String,
    designer: String,
    family_name: String,
    feature_prefixes: Vec<HashMap<String, serde_json::Value>>,
    features: Vec<HashMap<String, serde_json::Value>>,
    font_master: Vec<HashMap<String, serde_json::Value>>,
    glyphs: Vec<HashMap<String, serde_json::Value>>,
    instances: Vec<HashMap<String, serde_json::Value>>,
    units_per_em: String,
    user_data: HashMap<String, serde_json::Value>,
    version_major: String,
    version_minor: String,
}

fn main() -> std::io::Result<()> {
    let mut data_file = File::open("data/Cantarell.json")?;
    let mut contents = String::new();
    data_file.read_to_string(&mut contents)?;

    let font: Font = serde_json::from_str(&contents).unwrap();
    println!("{:?}", font);

    Ok(())
}
