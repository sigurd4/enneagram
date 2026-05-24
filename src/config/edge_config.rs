use core::ops::Deref;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EdgeConfig
{
    pub name: String,
    pub pivot: String,
    pub digit: Digit
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