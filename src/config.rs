use core::str::FromStr;
use std::{borrow::Cow, path::{Path, PathBuf}};

use ratatui_3d::Rgb;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ShowConfig
{
    pub path_lines: bool,
    pub boundary_lines: bool,
    pub pivot_lines: bool,
    pub triad_lines: bool,
}

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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EdgeConfig
{
    pub name: String,
    pub pivot: String,
    #[serde(serialize_with = "serialize_digit", deserialize_with = "deserialize_digit")]
    pub digit: Box<[[i8; 2]]>
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EdgesConfig
{
    pub recovery: EdgeConfig,
    pub association: EdgeConfig,
    pub repression: EdgeConfig,
    pub rejection: EdgeConfig,
    pub catatonia: EdgeConfig,
    pub paranoia: EdgeConfig,
    pub disorganization: EdgeConfig,
    pub action: EdgeConfig,
    pub rest: EdgeConfig
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TriadConfig
{
    pub description: String,
    pub expression: String,
    pub reflection: String,
    pub affirmation: String
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TriadsConfig
{
    pub positive: TriadConfig,
    pub competent: TriadConfig,
    pub reactive: TriadConfig,
    pub gut: TriadConfig,
    pub head: TriadConfig,
    pub heart: TriadConfig,
    pub assertive: TriadConfig,
    pub compliant: TriadConfig,
    pub withdrawn: TriadConfig,
    pub attachment: TriadConfig,
    pub frustration: TriadConfig,
    pub rejection: TriadConfig
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DomainConfig
{
    pub introverted_dissonance: String,
    pub introverted_synthesis: String,
    pub desire_machine: String,
    pub body_without_organs: String,
    pub extroverted_synthesis: String,
    pub extroverted_dissonance: String
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EnneagramConfig
{
    pub triads: TriadsConfig,
    pub edges: EdgesConfig,
    pub domains: DomainConfig,
    pub affirmation: String
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Config
{
    pub show: ShowConfig,
    pub color: ColorConfig,
    pub enneagram: EnneagramConfig
}

impl Default for ShowConfig
{
    fn default() -> Self
    {
        Self {
            path_lines: true,
            boundary_lines: true,
            pivot_lines: true,
            triad_lines: true,
        }
    }
}

impl Default for ColorConfig
{
    fn default() -> Self
    {
        Self {
            surface: Rgb(255, 255, 255),
            wire: Rgb(255, 0, 0),
            dyed: Rgb(255, 255, 255/2),
            glare: Rgb(255, 255, 255),
            sun: Rgb(255, 255, 255)
        }
    }
}

impl Default for EdgesConfig
{
    fn default() -> Self
    {
        Self {
            recovery: EdgeConfig {
                name: "Recovery".into(),
                pivot: "how will you manage your frustration?".into(),
                digit: [[-1, 3], [0, 4], [0, -4], [-1, -4], [1, -4]].into()
            },
            association: EdgeConfig {
                name: "Association".into(),
                pivot: "how will you gain any worth?".into(),
                digit: [[-3, 3], [-2, 4], [2, 4], [3, 3], [3, 0], [-3, -4], [3, -4], [3, -3]].into()
            },
            repression: EdgeConfig {
                name: "Repression".into(),
                pivot: "how will you repress your shame?".into(),
                digit: [[-3, 3], [-2, 4], [2, 4], [3, 3], [3, 1], [1, 0], [3, -1], [3, -3], [2, -4], [-2, -4], [-3, -3]].into()
            },
            rejection: EdgeConfig {
                name: "Rejection".into(),
                pivot: "how will you deal with your longing?".into(),
                digit: [[3, 1], [-3, 1], [1, 4], [1, -4]].into()
            },
            catatonia: EdgeConfig {
                name: "Catatonia".into(),
                pivot: "how will you gain any security?".into(),
                digit: [[3, 4], [-3, 4], [-3, 1], [2, 1], [3, 0], [3, -3], [2, -4], [-3, -4]].into()
            },
            paranoia: EdgeConfig {
                name: "Paranoia".into(),
                pivot: "how will you deal with your insecurity?".into(),
                digit: [[3, 4], [0, 4], [-3, -1], [-2, 0], [2, 0], [3, -1], [3, -3], [2, -4], [-2, -4], [-3, -3], [-3, -1]].into()
            },
            disorganization: EdgeConfig {
                name: "Disorganization".into(),
                pivot: "how will you handle your fear?".into(),
                digit: [[-3, 4], [3, 4], [-3, -4]].into()
            },
            action: EdgeConfig {
                name: "Action".into(),
                pivot: "how will you gain any control?".into(),
                digit: [[1, 0], [3, 1], [3, 3], [2, 4], [-2, 4], [-3, 3], [-3, 1], [-1, 0], [-3, -1], [-3, -3], [-2, -4], [2, -4], [3, -3], [3, -1], [1, 0], [-1, 0]].into()
            },
            rest: EdgeConfig {
                name: "Rest".into(),
                pivot: "how will you suppress your anger?".into(),
                digit: [[3, 1], [2, 0], [-2, 0], [-3, 1], [-3, 3], [-2, 4], [2, 4], [3, 3], [3, 1], [-3, -4]].into()
            }
        }
    }
}

impl Default for TriadsConfig
{
    fn default() -> Self
    {
        Self {
            positive: TriadConfig {
                description: "Positive/\"everything is fine\"".into(),
                expression: "everything is fine".into(),
                reflection: "you tell yourself that everything is fine".into(),
                affirmation: "stay positive".into()
            },
            competent: TriadConfig {
                description: "Competent/\"I take responsibility\"".into(),
                expression: "I take responsibility".into(),
                reflection: "you hold yourself responsible".into(),
                affirmation: "take responsibility".into()
            },
            reactive: TriadConfig {
                description: "Reactive/\"it's their fault\"".into(),
                expression: "it's their fault".into(),
                reflection: "you blame others".into(),
                affirmation: "blame others".into()
            },
            gut: TriadConfig {
                description: "Gut/\"I am my urges, my concience hurts me\"".into(),
                expression: "I am my urges, my concience hurts me".into(),
                reflection: "you have become your urges, your conscience hurts you".into(),
                affirmation: "follow my gut".into()
            },
            head: TriadConfig {
                description: "Head/\"I am my thoughts, my fear hurts me\"".into(),
                expression: "I am my thoughts, my fear hurts me".into(),
                reflection: "you have become your thoughts, your fear hurts you".into(),
                affirmation: "use my head".into()
            },
            heart: TriadConfig {
                description: "Heart/\"I am my emotions, my feelings hurt me\"".into(),
                expression: "I am my emotions, my feelings hurt me".into(),
                reflection: "you have become your emotions, your feelings hurt you".into(),
                affirmation: "follow my heart".into()
            },
            assertive: TriadConfig {
                description: "Assertive/\"I can change it\"".into(),
                expression: "I can change it".into(),
                reflection: "you believe you can change it".into(),
                affirmation: "change it".into()
            },
            compliant: TriadConfig {
                description: "Compliant/\"I can tolerate it\"".into(),
                expression: "I can tolerate it".into(),
                reflection: "you believe you can tolerate it".into(),
                affirmation: "tolerate it".into()
            },
            withdrawn: TriadConfig {
                description: "Withdrawn/\"I can avoid it\"".into(),
                expression: "I can avoid it".into(),
                reflection: "you believe you can avoid it".into(),
                affirmation: "avoid it".into()
            },
            attachment: TriadConfig {
                description: "Attachment/\"I need freedom\"".into(),
                expression: "I need freedom".into(),
                reflection: "you crave freedom".into(),
                affirmation: "be free".into()
            },
            frustration: TriadConfig {
                description: "Frustration/\"I need control\"".into(),
                expression: "I need control".into(),
                reflection: "you crave control".into(),
                affirmation: "be in control".into()
            },
            rejection: TriadConfig {
                description: "Rejection/\"I need love\"".into(),
                expression: "I need love".into(),
                reflection: "you crave love".into(),
                affirmation: "be accepted".into()
            }
        }
    }
}

impl Default for DomainConfig
{
    fn default() -> Self
    {
        Self {
            introverted_dissonance: "introverted dissonance".into(),
            introverted_synthesis: "introverted synthesis".into(),
            desire_machine: "desire-machine".into(),
            body_without_organs: "body without organs".into(),
            extroverted_synthesis: "extroverted synthesis".into(),
            extroverted_dissonance: "extroverted dissonance".into()
        }
    }
}

impl Default for EnneagramConfig
{
    fn default() -> Self
    {
        Self {
            triads: Default::default(),
            edges: Default::default(),
            domains: Default::default(),
            affirmation: "i will {}, {}, and {}.".into()
        }
    }
}

impl Default for Config
{
    fn default() -> Self
    {
        let default_config_path = Self::config_path("default.yaml");
        if !default_config_path.exists()
        {
            let default_default_config = Config {
                show: Default::default(),
                color: Default::default(),
                enneagram: Default::default()
            };
            default_default_config.write_config_path(&default_config_path);
            return default_default_config
        }
        Self::read_config_path(&default_config_path)
    }
}

impl Config
{
    pub fn config_dir() -> PathBuf
    {
        let xdg_config_home = std::env::var("XDG_CONFIG_HOME")
            .expect("Unable to locate directory for configuration: variable '$XDG_CONFIG_HOME' not defined.");
        let xdg_config_home_dir = PathBuf::from_str(&xdg_config_home)
            .expect(&format!("Unable to parse variable '$XDG_CONFIG_HOME' i.e. {xdg_config_home}."));
        assert!(xdg_config_home_dir.is_dir(), "'$XDG_CONFIG_HOME' i.e. '{}' isn't a directory.", xdg_config_home_dir.to_string_lossy());
        let config_dir = xdg_config_home_dir.join(Path::new("enneagram"));
        if !config_dir.exists()
        {
            std::fs::create_dir(&config_dir)
                .expect(&format!("Unable to create configuration directory '{}'.", config_dir.to_string_lossy()))
        }
        assert!(config_dir.is_dir(), "Configuration directory i.e. '{}' isn't a directory.", config_dir.to_string_lossy());
        config_dir
    }

    pub fn config_path<'a>(config: &'a str) -> Cow<'a, Path>
    {
        let mut config_path = Cow::from(Path::new(config));
        if config_path.file_name()
            .expect(&format!("Configuration '{}' has no filename", config_path.to_string_lossy()))
            == config_path.as_os_str()
        {
            let config_dir = Self::config_dir();
            config_path = config_dir.join(&config_path).into();
        }
        let _ = config_path.extension()
            .and_then(|extension| extension.to_str())
            .filter(|extension| *extension == "yaml")
            .expect(&format!("Configuration '{}' should have extension '.yaml'", config_path.to_string_lossy()));
        config_path
    }

    fn write_config_path(&self, config_path: &std::path::Path)
    {
        let yaml = serde_saphyr::to_string(self)
            .expect(&format!("Unable to serialize configuration '{}'.", config_path.to_string_lossy()));
        std::fs::write(config_path, yaml)
            .expect(&format!("Unable to create configuration '{}'.", config_path.to_string_lossy()))
    }

    fn read_config_path(config_path: &std::path::Path) -> Config
    {
        let yaml = std::fs::read_to_string(config_path)
            .expect(&format!("Unable to serialize configuration '{}'.", config_path.to_string_lossy()));
        serde_saphyr::from_str(&yaml)
        //serde_saphyr::from_str_validate(&yaml) // TODO: implement validator
            .expect(&format!("Unable to parse configuration '{}'.", config_path.to_string_lossy()))
    }

    #[allow(unused)]
    pub fn write_config(self, config: &str)
    {
        self.write_config_path(&Self::config_path(config))
    }

    pub fn read_config(config: &str) -> Config
    {
        Self::read_config_path(&Self::config_path(config))
    }
}

#[cfg(test)]
mod test
{
    use ratatui_3d::Rgb;

use crate::config::{Config, rgb_to_string, str_to_rgb};

    #[test]
    fn test_serde()
    {
        let config = Config::default();

        let yaml = serde_saphyr::to_string(&config).expect("Serialization failed.");

        println!("{yaml}");

        let config2 = serde_saphyr::from_str(&yaml).expect("Deserialization failed.");

        assert_eq!(config, config2, "Serialized then deserialized config differ from original.")
    }

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