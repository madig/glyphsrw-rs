use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::Read;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Font {
    #[serde(rename = ".appVersion")]
    app_version: String,
    copyright: Option<String>,
    custom_parameters: Option<Vec<CustomParameter>>,
    date: String,
    designer: Option<String>,
    family_name: String,
    feature_prefixes: Option<Vec<Feature>>,
    features: Option<Vec<Feature>>,
    font_master: Vec<FontMaster>,
    glyphs: Vec<Master>,
    instances: Option<Vec<Instance>>,
    units_per_em: String,
    user_data: Option<HashMap<String, serde_json::Value>>,
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
    alignment_zones: Option<Vec<String>>,
    ascender: String,
    cap_height: String,
    custom_parameters: Option<Vec<CustomParameter>>,
    descender: String,
    horizontal_stems: Option<Vec<String>>,
    id: String,
    user_data: Option<HashMap<String, serde_json::Value>>,
    vertical_stems: Option<Vec<String>>,
    weight_value: Option<String>,
    weight: Option<String>,
    x_height: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Instance {
    custom_parameters: Option<Vec<CustomParameter>>,
    exports: Option<String>,
    instance_interpolations: HashMap<String, String>,
    interpolation_weight: Option<String>,
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
    // let mut data_file = File::open("data/NotoSans-MM.json")?;
    // let mut data_file = File::open("data/UFOFilenameTest.json")?;
    // let mut data_file = File::open("data/IntegerFloat.json")?;
    // let mut data_file = File::open("data/GlyphsUnitTestSans.json")?;
    // let mut data_file = File::open("data/BracketTestFont2.json")?;
    // let mut data_file = File::open("data/BraceTestFont.json")?;
    // let mut data_file = File::open("data/AnchorAttachmentTest.json")?;
    let mut data_file = File::open("data/NewFont.json")?;
    // let mut data_file = File::open("data/Empty.json")?;
    let mut contents = String::new();
    data_file.read_to_string(&mut contents)?;

    let font: Font = serde_json::from_str(&contents).unwrap();
    println!(
        "{}",
        ron::ser::to_string_pretty(&font, ron::ser::PrettyConfig::default()).unwrap()
    );

    Ok(())
}
