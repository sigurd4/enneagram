use core::{borrow::Borrow, convert::Infallible, ops::Deref, str::FromStr};

#[cfg(feature = "artwork")]
use ratatui_3d::Rgb;

#[cfg(not(feature = "artwork"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Rgb(u8, u8, u8);

use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ColorConfig
{
    #[serde(skip_serializing_if = "Option::is_none")]
    sky: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    surface: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wire: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dyed: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    glare: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sun: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shine: Option<Color>
}

#[cfg(feature = "artwork")]
impl ColorConfig
{
    crate::config::getter!([_, c.color].sky.deref() -> Rgb);
    crate::config::getter!([_, c.color].surface.deref() -> Rgb);
    crate::config::getter!([_, c.color].wire.deref() -> Rgb);
    crate::config::getter!([_, c.color].dyed.deref() -> Rgb);
    crate::config::getter!([_, c.color].glare.deref() -> Rgb);
    crate::config::getter!([_, c.color].sun.deref() -> Rgb);
    crate::config::getter!([_, c.color].shine.deref() -> Rgb);

    pub fn line(&self, is_dyed: bool) -> Rgb
    {
        if is_dyed
        {
            self.dyed()
        }
        else
        {
            self.wire()
        }
    }
}

impl Borrow<ColorConfig> for Config
{
    fn borrow(&self) -> &ColorConfig
    {
        self.color()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Color(Rgb);

#[cfg(feature = "artwork")]
impl Deref for Color
{
    type Target = Rgb;

    fn deref(&self) -> &Self::Target
    {
        &self.0
    }
}

impl ToString for Color
{
    fn to_string(&self) -> String
    {
        let Self(Rgb(r, g, b)) = self;

        format!("{r:02X}{g:02X}{b:02X}")
    }
}

impl FromStr for Color
{
    type Err = Infallible;

    fn from_str(src: &str) -> Result<Self, Self::Err>
    {
        let rgb = u32::from_str_radix(src, 16)
            .expect(&format!("Unable to parse RBG hexadecimal color '{src}'."));

        assert!(rgb <= 0xFFFFFF, "RGB color cannot have alpha-channel.");

        Ok(Self(Rgb(
            (rgb >> 16) as u8,
            (rgb >> 8) as u8,
            rgb as u8
        )))
    }
}

impl Serialize for Color
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Color
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        Self::from_str(&String::deserialize(deserializer)?)
            .map_err(|err| match err {})
    }
}

#[cfg(test)]
mod test
{
    use core::str::FromStr;

    use super::Rgb;

    use crate::config::color_config::Color;

    #[test]
    fn test_hex()
    {
        let rgb = Color(Rgb(0, 10, 0));

        let hex = rgb.to_string();

        println!("{}", hex);

        let rgb_decoded = Color::from_str(&hex).unwrap();

        assert_eq!(rgb, rgb_decoded)
    }
}