use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::Read;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Font {
    #[serde(rename = ".appVersion")]
    app_version: String,
    copyright: String,
    custom_parameters: Vec<CustomParameter>,
    date: String,
    designer: String,
    family_name: String,
    feature_prefixes: Vec<Feature>,
    features: Vec<Feature>,
    font_master: Vec<FontMaster>,
    glyphs: Vec<Master>,
    instances: Vec<Instance>,
    units_per_em: String,
    user_data: HashMap<String, serde_json::Value>,
    version_major: String,
    version_minor: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct CustomParameter {
    name: String,
    value: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
struct Feature {
    automatic: Option<String>,
    code: String,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct FontMaster {
    alignment_zones: Vec<String>,
    ascender: String,
    cap_height: String,
    custom_parameters: Vec<CustomParameter>,
    descender: String,
    horizontal_stems: Vec<String>,
    id: String,
    user_data: Option<HashMap<String, serde_json::Value>>,
    vertical_stems: Vec<String>,
    weight_value: String,
    weight: Option<String>,
    x_height: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Instance {
    custom_parameters: Option<Vec<CustomParameter>>,
    exports: Option<String>,
    instance_interpolations: HashMap<String, String>,
    interpolation_weight: String,
    name: String,
    weight_class: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Master {
    glyphname: String,
    layers: Vec<Layer>,
    note: Option<String>,
    unicode: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Layer {
    anchors: Option<Vec<Anchor>>,
    layer_id: String,
    paths: Option<Vec<Path>>,
    width: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Anchor {
    name: String,
    position: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Path {
    closed: String,
    nodes: Vec<String>,
}

fn main() -> std::io::Result<()> {
    // let mut data_file = File::open("data/Cantarell.json")?;
    // let mut data_file = File::open("data/WorkSans.json")?;
    // let mut data_file = File::open("data/WorkSans-Italic.json")?;
    let mut data_file = File::open("data/NotoSans-MM.json")?;
    let mut contents = String::new();
    data_file.read_to_string(&mut contents)?;

    let font: Font = serde_json::from_str(&contents).unwrap();
    println!("{:?}", font);

    Ok(())
}
