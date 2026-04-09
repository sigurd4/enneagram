use crate::triad::Triad;

/// Homonculus of the self/internalization of self/frame of judgement/meta-objective/"Who am i?"
#[derive(Clone, Copy, PartialEq, Eq)]
#[derive(enum_display::EnumDisplay)]
pub enum Frame
{
    #[display("I am my urges, my concience hurts me")]
    Gut,
    #[display("I am my thoughts, my fear hurts me")]
    Head,
    #[display("I am my emotions, my feelings hurt me")]
    Heart
}

impl Triad for Frame
{
    fn expression(&self) -> &'static str
    {
        match self
        {
            Self::Gut => "I am my urges, my concience hurts me",
            Self::Head => "I am my thoughts, my fear hurts me",
            Self::Heart => "I am my emotions, my feelings hurt me"
        }
    }

    fn reflection(&self) -> &'static str
    {
        match self
        {
            Self::Gut => "you have become your urges, your conscience hurts you",
            Self::Head => "you have become your thoughts, your fear hurts you",
            Self::Heart => "you have become your emotions, your feelings hurt you"
        }
    }
}