use core::{borrow::Borrow, convert::Infallible, ops::Deref, str::FromStr};

use ratatui_3d::Rgb;
use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ColorConfig
{
    #[serde(skip_serializing_if = "Option::is_none")]
    surface: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wire: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dyed: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    glare: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sun: Option<Color>
}

impl ColorConfig
{
    pub fn surface(&self) -> &Rgb
    {
        crate::config::member!([self, c.color].surface).deref()
    }
    pub fn wire(&self) -> &Rgb
    {
        crate::config::member!([self, c.color].wire).deref()
    }
    pub fn dyed(&self) -> &Rgb
    {
        crate::config::member!([self, c.color].dyed).deref()
    }
    pub fn glare(&self) -> &Rgb
    {
        crate::config::member!([self, c.color].glare).deref()
    }
    pub fn sun(&self) -> &Rgb
    {
        crate::config::member!([self, c.color].sun).deref()
    }

    pub fn line(&self, is_dyed: bool) -> &Rgb
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

use ratatui_3d::Rgb;

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