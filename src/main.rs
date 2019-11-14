use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::Read;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
struct Font {
    #[serde(rename = ".appVersion")]
    app_version: String,
    classes: Option<Vec<Feature>>,
    copyright: Option<String>,
    custom_parameters: Option<Vec<CustomParameter>>,
    #[serde(rename = "DisplayStrings")]
    display_strings: Option<Vec<String>>,
    date: String,
    designer: Option<String>,
    family_name: String,
    feature_prefixes: Option<Vec<Feature>>,
    features: Option<Vec<Feature>>,
    font_master: Vec<FontMaster>,
    glyphs: Vec<Master>,
    instances: Option<Vec<Instance>>,
    kerning: Option<HashMap<String, HashMap<String, HashMap<String, String>>>>,
    units_per_em: String,
    user_data: Option<HashMap<String, serde_json::Value>>,
    version_major: String,
    version_minor: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct CustomParameter {
    name: String,
    value: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct Guideline {
    angle: Option<String>,
    locked: Option<String>,
    position: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct Feature {
    automatic: Option<String>,
    code: String,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
struct FontMaster {
    alignment_zones: Option<Vec<String>>,
    ascender: String,
    cap_height: String,
    custom_parameters: Option<Vec<CustomParameter>>,
    descender: String,
    guide_lines: Option<Vec<Guideline>>,
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
#[serde(deny_unknown_fields)]
struct Instance {
    custom_parameters: Option<Vec<CustomParameter>>,
    exports: Option<String>,
    instance_interpolations: HashMap<String, String>,
    interpolation_weight: Option<String>,
    manual_interpolation: Option<String>,
    name: String,
    weight_class: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
struct Master {
    category: Option<String>,
    color: Option<String>,
    export: Option<String>,
    glyphname: String,
    last_change: Option<String>,
    parts_settings: Option<Vec<PartsSetting>>,
    layers: Vec<Layer>,
    left_kerning_group: Option<String>,
    left_metrics_key: Option<String>,
    note: Option<String>,
    right_kerning_group: Option<String>,
    right_metrics_key: Option<String>,
    script: Option<String>,
    sub_category: Option<String>,
    unicode: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
struct Layer {
    anchors: Option<Vec<Anchor>>,
    annotations: Option<Vec<Annotation>>,
    associated_master_id: Option<String>,
    background_image: Option<BackgroundImage>,
    background: Option<BackgroundLayer>,
    components: Option<Vec<Component>>,
    guide_lines: Option<Vec<Guideline>>,
    hints: Option<Vec<Hint>>,
    layer_id: String,
    name: Option<String>,
    paths: Option<Vec<Path>>,
    user_data: Option<HashMap<String, serde_json::Value>>,
    width: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
struct BackgroundLayer {
    anchors: Option<Vec<Anchor>>,
    components: Option<Vec<Component>>,
    layer_id: Option<String>,
    paths: Option<Vec<Path>>,
    width: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct Anchor {
    name: String,
    position: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct Component {
    name: String,
    transform: Option<String>,
    piece: Option<ComponentPiece>,
    anchor: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
struct ComponentPiece {
    height: Option<String>,
    crotch_depth: Option<String>,
    shoulder_width: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
struct PartsSetting {
    bottom_name: String,
    bottom_value: String,
    name: String,
    top_name: String,
    top_value: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct Annotation {
    angle: Option<String>,
    position: String,
    text: Option<String>,
    width: Option<String>,
    r#type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
struct BackgroundImage {
    crop: String,
    image_path: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct Hint {
    horizontal: String,
    origin: String,
    target: String,
    r#type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
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
    let mut data_file = File::open("data/AnchorAttachmentTest.json")?;
    // let mut data_file = File::open("data/NewFont.json")?;
    // let mut data_file = File::open("data/Empty.json")?;
    let mut contents = String::new();
    data_file.read_to_string(&mut contents)?;

    let font: Font = serde_json::from_str(&contents).unwrap();
    println!("{}", ron::ser::to_string_pretty(&font, ron::ser::PrettyConfig::default()).unwrap());

    Ok(())
}
