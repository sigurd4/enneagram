use ratatui_3d::Rgb;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ColorConfig
{
    #[serde(serialize_with = "serialize_rgb", deserialize_with = "deserialize_rgb")]
    pub surface: Rgb,
    #[serde(serialize_with = "serialize_rgb", deserialize_with = "deserialize_rgb")]
    pub wire: Rgb,
    #[serde(serialize_with = "serialize_rgb", deserialize_with = "deserialize_rgb")]
    pub dyed: Rgb,
    #[serde(serialize_with = "serialize_rgb", deserialize_with = "deserialize_rgb")]
    pub glare: Rgb,
    #[serde(serialize_with = "serialize_rgb", deserialize_with = "deserialize_rgb")]
    pub sun: Rgb
}

fn rgb_to_string(value: &Rgb) -> String
{
    let Rgb(r, g, b) = value;

    format!("{r:02X}{g:02X}{b:02X}")
}
fn str_to_rgb(src: &str) -> Rgb
{
    let rgb = u32::from_str_radix(src, 16)
        .expect(&format!("Unable to parse RBG hexadecimal color '{src}'."));

    assert!(rgb <= 0xFFFFFF, "RGB color cannot have alpha-channel.");

    Rgb(
        (rgb >> 16) as u8,
        (rgb >> 8) as u8,
        rgb as u8
    )
}

fn serialize_rgb<S>(value: &Rgb, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer
{
    let hex = rgb_to_string(value);

    hex.serialize(serializer)
}
fn deserialize_rgb<'de, D>(deserializer: D) -> Result<Rgb, D::Error>
where
    D: Deserializer<'de>
{
    let src = String::deserialize(deserializer)?;

    Ok(str_to_rgb(&src))
}

#[cfg(test)]
mod test
{
    use ratatui_3d::Rgb;

use crate::config::color_config::{rgb_to_string, str_to_rgb};

    #[test]
    fn test_hex()
    {
        let rgb = Rgb(0, 10, 0);

        let hex = rgb_to_string(&rgb);

        println!("{}", hex);

        let rgb_decoded = str_to_rgb(&hex);

        assert_eq!(rgb, rgb_decoded)
    }
}