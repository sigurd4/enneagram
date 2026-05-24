use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EdgeConfig
{
    pub name: String,
    pub pivot: String,
    #[serde(serialize_with = "serialize_digit", deserialize_with = "deserialize_digit")]
    pub digit: Box<[[i8; 2]]>
}

fn serialize_digit<S>(value: &Box<[[i8; 2]]>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer
{
    value.iter()
        .map(|[x, y]| format!("{x},{y}"))
        .collect::<Vec<_>>()
        .join(" ")
        .serialize(serializer)
}
fn deserialize_digit<'de, D>(deserializer: D) -> Result<Box<[[i8; 2]]>, D::Error>
where
    D: Deserializer<'de>
{
    Ok(
        String::deserialize(deserializer)?
            .split(" ")
            .map(|xy| xy.split(",")
                .map(|e| i8::from_str_radix(e, 10).expect("Failed parsing 8-bit int coordinate of digit."))
                .collect::<Vec<_>>()
                .try_into()
                .expect("Each point of the digit must contain exactly two coordinates.")
            ).collect::<Vec<_>>()
            .into()
    )
}