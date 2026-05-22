use ratatui_3d::Rgb;

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
pub struct ColorConfig
{
    pub surface: Rgb,
    pub wire: Rgb,
    pub dyed: Rgb,
    pub glare: Rgb,
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

#[derive(Clone, Copy)]
pub struct EdgeConfig<'a>
{
    pub name: &'a str,
    pub digit: &'a [[i8; 2]]
}

#[derive(Clone, Copy)]
pub struct EdgesConfig<'a>
{
    pub recovery: EdgeConfig<'a>,
    pub association: EdgeConfig<'a>,
    pub repression: EdgeConfig<'a>,
    pub rejection: EdgeConfig<'a>,
    pub catatonia: EdgeConfig<'a>,
    pub paranoia: EdgeConfig<'a>,
    pub disorganization: EdgeConfig<'a>,
    pub action: EdgeConfig<'a>,
    pub rest: EdgeConfig<'a>
}

impl Default for EdgesConfig<'_>
{
    fn default() -> Self
    {
        Self {
            recovery: EdgeConfig {
                name: "Recovery",
                digit: &[[-1, 3], [0, 4], [0, -4], [-1, -4], [1, -4]]
            },
            association: EdgeConfig {
                name: "Association",
                digit: &[[-3, 3], [-2, 4], [2, 4], [3, 3], [3, 0], [-3, -4], [3, -4], [3, -3]]
            },
            repression: EdgeConfig {
                name: "Repression",
                digit: &[[-3, 3], [-2, 4], [2, 4], [3, 3], [3, 1], [1, 0], [3, -1], [3, -3], [2, -4], [-2, -4], [-3, -3]]
            },
            rejection: EdgeConfig {
                name: "Rejection",
                digit: &[[3, 1], [-3, 1], [1, 4], [1, -4]]
            },
            catatonia: EdgeConfig {
                name: "Catatonia",
                digit: &[[3, 4], [-3, 4], [-3, 1], [2, 1], [3, 0], [3, -3], [2, -4], [-3, -4]]
            },
            paranoia: EdgeConfig {
                name: "Paranoia",
                digit: &[[3, 4], [0, 4], [-3, -1], [-2, 0], [2, 0], [3, -1], [3, -3], [2, -4], [-2, -4], [-3, -3], [-3, -1]]
            },
            disorganization: EdgeConfig {
                name: "Disorganization",
                digit: &[[-3, 4], [3, 4], [-3, -4]]
            },
            action: EdgeConfig {
                name: "Action",
                digit: &[[1, 0], [3, 1], [3, 3], [2, 4], [-2, 4], [-3, 3], [-3, 1], [-1, 0], [-3, -1], [-3, -3], [-2, -4], [2, -4], [3, -3], [3, -1], [1, 0], [-1, 0]]
            },
            rest: EdgeConfig {
                name: "Rest",
                digit: &[[3, 1], [2, 0], [-2, 0], [-3, 1], [-3, 3], [-2, 4], [2, 4], [3, 3], [3, 1], [-3, -4]]
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct TriadConfig<'a>
{
    pub description: &'a str,
    pub expression: &'a str,
    pub reflection: &'a str,
    pub affirmation: &'a str
}

#[derive(Clone, Copy)]
pub struct TriadsConfig<'a>
{
    pub positive: TriadConfig<'a>,
    pub competent: TriadConfig<'a>,
    pub reactive: TriadConfig<'a>,
    pub gut: TriadConfig<'a>,
    pub head: TriadConfig<'a>,
    pub heart: TriadConfig<'a>,
    pub assertive: TriadConfig<'a>,
    pub compliant: TriadConfig<'a>,
    pub withdrawn: TriadConfig<'a>,
    pub attachment: TriadConfig<'a>,
    pub frustration: TriadConfig<'a>,
    pub rejection: TriadConfig<'a>
}

impl Default for TriadsConfig<'_>
{
    fn default() -> Self
    {
        Self {
            positive: TriadConfig {
                description: "Positive/\"everything is fine\"",
                expression: "everything is fine",
                reflection: "you tell yourself that everything is fine",
                affirmation: "stay positive"
            },
            competent: TriadConfig {
                description: "Competent/\"I take responsibility\"",
                expression: "I take responsibility",
                reflection: "you hold yourself responsible",
                affirmation: "take responsibility"
            },
            reactive: TriadConfig {
                description: "Reactive/\"it's their fault\"",
                expression: "it's their fault",
                reflection: "you blame others",
                affirmation: "blame others"
            },
            gut: TriadConfig {
                description: "Gut/\"I am my urges, my concience hurts me\"",
                expression: "I am my urges, my concience hurts me",
                reflection: "you have become your urges, your conscience hurts you",
                affirmation: "follow my gut"
            },
            head: TriadConfig {
                description: "Head/\"I am my thoughts, my fear hurts me\"",
                expression: "I am my thoughts, my fear hurts me",
                reflection: "you have become your thoughts, your fear hurts you",
                affirmation: "use my head"
            },
            heart: TriadConfig {
                description: "Heart/\"I am my emotions, my feelings hurt me\"",
                expression: "I am my emotions, my feelings hurt me",
                reflection: "you have become your emotions, your feelings hurt you",
                affirmation: "follow my heart"
            },
            assertive: TriadConfig {
                description: "Assertive/\"I can change it\"",
                expression: "I can change it",
                reflection: "you believe you can change it",
                affirmation: "change it"
            },
            compliant: TriadConfig {
                description: "Compliant/\"I can tolerate it\"",
                expression: "I can tolerate it",
                reflection: "you believe you can tolerate it",
                affirmation: "tolerate it"
            },
            withdrawn: TriadConfig {
                description: "Withdrawn/\"I can avoid it\"",
                expression: "I can avoid it",
                reflection: "you believe you can avoid it",
                affirmation: "avoid it"
            },
            attachment: TriadConfig {
                description: "Attachment/\"I need freedom\"",
                expression: "I need freedom",
                reflection: "you crave freedom",
                affirmation: "be free"
            },
            frustration: TriadConfig {
                description: "Frustration/\"I need control\"",
                expression: "I need control",
                reflection: "you crave control",
                affirmation: "be in control"
            },
            rejection: TriadConfig {
                description: "Rejection/\"I need love\"",
                expression: "I need love",
                reflection: "you crave love",
                affirmation: "be accepted"
            }
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct DomainConfig<'a>
{
    pub introverted_dissonance: &'a str,
    pub introverted_synthesis: &'a str,
    pub desire_machine: &'a str,
    pub body_without_organs: &'a str,
    pub extroverted_synthesis: &'a str,
    pub extroverted_dissonance: &'a str
}

#[derive(Clone, Copy, Default)]
pub struct EnneagramConfig<'a>
{
    pub triads: TriadsConfig<'a>,
    pub edges: EdgesConfig<'a>,
    pub domains: DomainConfig<'a>,
    pub affirmation: &'a str
}

#[derive(Clone, Copy, Default)]
pub struct Config<'a>
{
    pub show: ShowConfig,
    pub color: ColorConfig,
    pub enneagram: EnneagramConfig<'a>
}