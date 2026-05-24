use core::borrow::Borrow;

use ratatui_3d::Rgb;
use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ColorConfig
{
    #[serde(with = "rgb")]
    pub surface: Rgb,
    #[serde(with = "rgb")]
    pub wire: Rgb,
    #[serde(with = "rgb")]
    pub dyed: Rgb,
    #[serde(with = "rgb")]
    pub glare: Rgb,
    #[serde(with = "rgb")]
    pub sun: Rgb
}

impl Borrow<ColorConfig> for Config
{
    fn borrow(&self) -> &ColorConfig
    {
        &self.color
    }
}

mod rgb
{
    use ratatui_3d::Rgb;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use self as rgb;

    pub(super) fn to_string(value: &Rgb) -> String
    {
        let Rgb(r, g, b) = value;

        format!("{r:02X}{g:02X}{b:02X}")
    }
    pub(super) fn from_str(src: &str) -> Rgb
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

    pub fn serialize<S>(value: &Rgb, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let hex = rgb::to_string(value);

        hex.serialize(serializer)
    }
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Rgb, D::Error>
    where
        D: Deserializer<'de>
    {
        let src = String::deserialize(deserializer)?;

        Ok(rgb::from_str(&src))
    }
}

#[cfg(test)]
mod test
{
    use ratatui_3d::Rgb;

    use crate::config::color_config::rgb;

    #[test]
    fn test_hex()
    {
        let rgb = Rgb(0, 10, 0);

        let hex = rgb::to_string(&rgb);

        println!("{}", hex);

        let rgb_decoded = rgb::from_str(&hex);

        assert_eq!(rgb, rgb_decoded)
    }
}