use core::ops::{BitOrAssign, Deref};
use std::borrow::Cow;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PartialEdgeConfig<'a>
{
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pivot: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    digit: Option<Cow<'a, Digit>>
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EdgeConfig<'a>
{
    pub name: Cow<'a, str>,
    pub pivot: Cow<'a, str>,
    pub digit: Cow<'a, Digit>
}

impl<'a> BitOrAssign<&'a PartialEdgeConfig<'a>> for PartialEdgeConfig<'a>
{
    fn bitor_assign(&mut self, rhs: &'a PartialEdgeConfig<'a>)
    {
        let Self { name, pivot, digit } = rhs;

        if let Some(v) = name.as_ref() { self.name.get_or_insert_with(|| v.as_ref().into()); }
        if let Some(v) = pivot.as_ref() { self.pivot.get_or_insert_with(|| v.as_ref().into()); }
        if let Some(v) = digit.as_ref() { self.digit.get_or_insert_with(|| Cow::Borrowed(v.as_ref())); }
    }
}

impl<'a> From<&'a PartialEdgeConfig<'a>> for PartialEdgeConfig<'a>
{
    fn from(value: &'a PartialEdgeConfig<'a>) -> Self
    {
        let Self { name, pivot, digit } = value;

        Self {
            name: name.as_ref().map(|c| c.as_ref().into()),
            pivot: pivot.as_ref().map(|c| c.as_ref().into()),
            digit: digit.as_ref().map(|c| Cow::Borrowed(c.as_ref()))
        }
    }
}

impl<'a> TryFrom<&'a PartialEdgeConfig<'a>> for EdgeConfig<'a>
{
    type Error = &'a PartialEdgeConfig<'a>;

    fn try_from(value: &'a PartialEdgeConfig<'a>) -> Result<Self, Self::Error>
    {
        let PartialEdgeConfig { name, pivot, digit } = value;

        Ok(Self {
            name: name.as_ref().ok_or(value)?.as_ref().into(),
            pivot: pivot.as_ref().ok_or(value)?.as_ref().into(),
            digit: Cow::Borrowed(digit.as_ref().ok_or(value)?.as_ref())
        })
    }
}
impl<'a> TryFrom<PartialEdgeConfig<'a>> for EdgeConfig<'a>
{
    type Error = PartialEdgeConfig<'a>;

    fn try_from(value: PartialEdgeConfig<'a>) -> Result<Self, Self::Error>
    {
        let PartialEdgeConfig { name, pivot, digit } = value.clone();

        Ok(Self {
            name: name.ok_or(value.clone())?,
            pivot: pivot.ok_or(value.clone())?,
            digit: digit.ok_or(value)?
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Digit(Box<[[i8; 2]]>);

impl Deref for Digit
{
    type Target = [[i8; 2]];

    fn deref(&self) -> &Self::Target
    {
        &self.0
    }
}

impl Serialize for Digit
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let Self(curve) = self;

        curve.iter()
            .map(|[x, y]| format!("{x},{y}"))
            .collect::<Vec<_>>()
            .join(" ")
            .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Digit
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        let curve = String::deserialize(deserializer)?
            .split(" ")
            .map(|xy| xy.split(",")
                .map(|e| i8::from_str_radix(e, 10).expect("Failed parsing 8-bit int coordinate of digit."))
                .collect::<Vec<_>>()
                .try_into()
                .expect("Each point of the digit must contain exactly two coordinates.")
            ).collect::<Vec<_>>()
            .into();

        Ok(Self(curve))
    }
}