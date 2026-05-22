use ratatui_3d::Rgb;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct ShowConfig
{
    pub path_lines: bool,
    pub boundary_lines: bool,
    pub pivot_lines: bool,
    pub triad_lines: bool,
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

fn serialize_rgb<S>(value: &Rgb, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer
{
    let Rgb(r, g, b) = value;

    rgb::Rgb { r, g, b }.serialize(serializer)
}
fn deserialize_rgb<'de, D>(deserializer: D) -> Result<Rgb, D::Error>
where
    D: Deserializer<'de>
{
    let rgb::Rgb {r, g, b} = rgb::Rgb::deserialize(deserializer)?;

    Ok(Rgb(r, g, b))
}

#[derive(Clone, Copy, Serialize, Deserialize)]
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

#[derive(Clone, Serialize, Deserialize)]
pub struct EdgeConfig
{
    pub name: String,
    pub digit: Box<[[i8; 2]]>
}

#[derive(Clone, Serialize, Deserialize)]
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

impl Default for EdgesConfig
{
    fn default() -> Self
    {
        Self {
            recovery: EdgeConfig {
                name: "Recovery".into(),
                digit: [[-1, 3], [0, 4], [0, -4], [-1, -4], [1, -4]].into()
            },
            association: EdgeConfig {
                name: "Association".into(),
                digit: [[-3, 3], [-2, 4], [2, 4], [3, 3], [3, 0], [-3, -4], [3, -4], [3, -3]].into()
            },
            repression: EdgeConfig {
                name: "Repression".into(),
                digit: [[-3, 3], [-2, 4], [2, 4], [3, 3], [3, 1], [1, 0], [3, -1], [3, -3], [2, -4], [-2, -4], [-3, -3]].into()
            },
            rejection: EdgeConfig {
                name: "Rejection".into(),
                digit: [[3, 1], [-3, 1], [1, 4], [1, -4]].into()
            },
            catatonia: EdgeConfig {
                name: "Catatonia".into(),
                digit: [[3, 4], [-3, 4], [-3, 1], [2, 1], [3, 0], [3, -3], [2, -4], [-3, -4]].into()
            },
            paranoia: EdgeConfig {
                name: "Paranoia".into(),
                digit: [[3, 4], [0, 4], [-3, -1], [-2, 0], [2, 0], [3, -1], [3, -3], [2, -4], [-2, -4], [-3, -3], [-3, -1]].into()
            },
            disorganization: EdgeConfig {
                name: "Disorganization".into(),
                digit: [[-3, 4], [3, 4], [-3, -4]].into()
            },
            action: EdgeConfig {
                name: "Action".into(),
                digit: [[1, 0], [3, 1], [3, 3], [2, 4], [-2, 4], [-3, 3], [-3, 1], [-1, 0], [-3, -1], [-3, -3], [-2, -4], [2, -4], [3, -3], [3, -1], [1, 0], [-1, 0]].into()
            },
            rest: EdgeConfig {
                name: "Rest".into(),
                digit: [[3, 1], [2, 0], [-2, 0], [-3, 1], [-3, 3], [-2, 4], [2, 4], [3, 3], [3, 1], [-3, -4]].into()
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TriadConfig
{
    pub description: String,
    pub expression: String,
    pub reflection: String,
    pub affirmation: String
}

#[derive(Clone, Serialize, Deserialize)]
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

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DomainConfig
{
    pub introverted_dissonance: String,
    pub introverted_synthesis: String,
    pub desire_machine: String,
    pub body_without_organs: String,
    pub extroverted_synthesis: String,
    pub extroverted_dissonance: String
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct EnneagramConfig
{
    pub triads: TriadsConfig,
    pub edges: EdgesConfig,
    pub domains: DomainConfig,
    pub affirmation: String
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Config
{
    pub show: ShowConfig,
    pub color: ColorConfig,
    pub enneagram: EnneagramConfig
}