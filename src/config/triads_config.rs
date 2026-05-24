use serde::{Deserialize, Serialize};

use crate::config::TriadConfig;

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