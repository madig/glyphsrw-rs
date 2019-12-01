use plist; // XXX: Disabled integer guess for now for testing.
use serde::de::{Deserializer, Visitor};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// use std::fs::File;
// use std::io::prelude::Read;

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
    #[serde(rename = "designerURL")]
    designer_url: Option<String>,
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
    filter: Option<String>,
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
    custom_value: Option<String>,
    custom: Option<String>,
    descender: String,
    guide_lines: Option<Vec<Guideline>>,
    horizontal_stems: Option<Vec<String>>,
    icon_name: Option<String>,
    id: String,
    user_data: Option<HashMap<String, serde_json::Value>>,
    vertical_stems: Option<Vec<String>>,
    weight_value: Option<String>,
    weight: Option<String>,
    width_value: Option<String>,
    width: Option<String>,
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
    interpolation_width: Option<String>,
    is_bold: Option<String>,
    link_style: Option<String>,
    manual_interpolation: Option<String>,
    name: String,
    weight_class: Option<String>,
    width_class: Option<String>,
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
    #[serde(default, deserialize_with = "deserialize_option_unicode")]
    unicode: Option<Vec<u32>>,
    // unicode: Option<String>,
}

fn deserialize_option_unicode<'de, D>(deserializer: D) -> Result<Option<Vec<u32>>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Wrapper(#[serde(deserialize_with = "deserialize_unicode")] Vec<u32>);

    let v = Option::deserialize(deserializer)?;
    Ok(v.map(|Wrapper(a)| a))
}

fn deserialize_unicode<'de, D>(deserializer: D) -> Result<Vec<u32>, D::Error>
where
    D: Deserializer<'de>,
{
    struct UnicodeVisitor;
    impl<'de> Visitor<'de> for UnicodeVisitor {
        type Value = Vec<u32>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string or int describing a Unicode value as a hex value.")
        }

        fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            self.visit_str(&format!("{:04}", v))
        }

        fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            let unicodes: Vec<u32> = s
                .split(",")
                .map(|e| u32::from_str_radix(e, 16).unwrap())
                .collect();
            Ok(unicodes)
        }
    }

    deserializer.deserialize_any(UnicodeVisitor)
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
    // let mut data_file = File::open("data/AnchorAttachmentTest.json")?;
    // let mut data_file = File::open("data/NewFont.json")?;
    // let mut data_file = File::open("data/Empty.json")?;
    // let mut contents = String::new();
    // data_file.read_to_string(&mut contents)?;

    // let font: Font = serde_json::from_str(&contents).unwrap();
    // println!(
    //     "{}",
    //     ron::ser::to_string_pretty(&font, ron::ser::PrettyConfig::default()).unwrap()
    // );

    // let f: Font = plist::from_file("data/Empty.glyphs").expect("lol");
    // let f: Font = plist::from_file("data/NewFont.glyphs").expect("lol");
    let f: Font = plist::from_file("data/AnchorAttachmentTest.glyphs").expect("lol");
    println!(
        "{}",
        ron::ser::to_string_pretty(&f, ron::ser::PrettyConfig::default()).unwrap()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty() -> Result<(), plist::Error> {
        let _f: Font = plist::from_file("data/Empty.glyphs")?;
        Ok(())
    }

    #[test]
    fn parse_new_font() -> Result<(), plist::Error> {
        let _f: Font = plist::from_file("data/NewFont.glyphs")?;
        Ok(())
    }

    #[test]
    fn parse_anchor_attachment() -> Result<(), plist::Error> {
        let _f: Font = plist::from_file("data/AnchorAttachmentTest.glyphs")?;
        Ok(())
    }

    #[test]
    fn parse_brace_test_font() -> Result<(), plist::Error> {
        let _f: Font = plist::from_file("data/BraceTestFont.glyphs")?;
        Ok(())
    }

    #[test]
    fn parse_bracket_test_font2() -> Result<(), plist::Error> {
        let _f: Font = plist::from_file("data/BracketTestFont2.glyphs")?;
        Ok(())
    }

    #[test]
    fn parse_glyphs_unit_test_sans() -> Result<(), plist::Error> {
        let _f: Font = plist::from_file("data/GlyphsUnitTestSans.glyphs")?;
        Ok(())
    }

    #[test]
    fn parse_integer_float() -> Result<(), plist::Error> {
        let _f: Font = plist::from_file("data/IntegerFloat.glyphs")?;
        Ok(())
    }

    #[test]
    fn parse_cantarell() -> Result<(), plist::Error> {
        let _f: Font = plist::from_file("data/Cantarell.glyphs")?;
        Ok(())
    }

    #[test]
    fn parse_worksans() -> Result<(), plist::Error> {
        let _f: Font = plist::from_file("data/WorkSans.glyphs")?;
        Ok(())
    }

    #[test]
    fn parse_worksans_italic() -> Result<(), plist::Error> {
        let _f: Font = plist::from_file("data/WorkSans-Italic.glyphs")?;
        Ok(())
    }

    #[test]
    fn parse_noto_sans_mm() -> Result<(), plist::Error> {
        let _f: Font = plist::from_file("data/NotoSans-MM.glyphs")?;
        Ok(())
    }

    #[test]
    fn parse_ufo_filename_test() -> Result<(), plist::Error> {
        let _f: Font = plist::from_file("data/UFOFilenameTest.glyphs")?;
        Ok(())
    }
}
