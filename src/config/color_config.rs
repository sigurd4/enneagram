use core::{ops::Deref, str::FromStr};
use std::{fmt::Display, num::ParseIntError};

#[cfg(feature = "artwork")]
use ratatui_3d::Rgb;

#[cfg(not(feature = "artwork"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Rgb(u8, u8, u8);

use serde::{Deserialize, Serialize};

#[cfg(feature = "artwork")]
use crate::config::Fallback;
use crate::config::{Config, Property};

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

    pub fn line(&self, fallback: &Fallback, is_dyed: bool) -> Rgb
    {
        if is_dyed
        {
            self.dyed(fallback)
        }
        else
        {
            self.wire(fallback)
        }
    }
}

impl Property<ColorConfig> for ColorConfig
{
    fn property<'a>(&'a self, _fallback: &'a Fallback) -> &'a ColorConfig
    {
        self
    }
}
impl<C> Property<ColorConfig> for C
where
    C: Property<Config>
{
    fn property<'a>(&'a self, fallback: &'a Fallback) -> &'a ColorConfig
    {
        self.property(fallback).color(fallback)
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

impl Display for Color
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let Self(Rgb(r, g, b)) = self;
        
        write!(f, "{r:02X}{g:02X}{b:02X}")
    }
}

#[derive(Debug)]
pub enum ParseColorError {
    ParseInt {
        data: String,
        error: ParseIntError
    },
    Transparent {
        data: String,
        rgba: u32
    }
}
impl Display for ParseColorError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            Self::ParseInt { data, error } => write!(f, "Unable to parse RBG hexadecimal color '{data}'. {error}"),
            Self::Transparent { data, rgba } => write!(f, "RGB color '{data}' a.k.a. #{rgba:#010x} cannot have alpha-channel.")
        }
    }
}

impl FromStr for Color
{
    type Err = ParseColorError;

    fn from_str(src: &str) -> Result<Self, Self::Err>
    {
        let rgb = u32::from_str_radix(src, 16).map_err(|error| ParseColorError::ParseInt { data: src.to_string(), error })?;

        if rgb > 0xFFFFFF
        {
            return Err(ParseColorError::Transparent { data: src.to_string(), rgba: rgb })
        }

        Ok(Self(Rgb((rgb >> 16) as u8, (rgb >> 8) as u8, rgb as u8)))
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
        Self::from_str(&String::deserialize(deserializer)?).map_err(|err| serde::de::Error::custom(err.to_string()))
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
