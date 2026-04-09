use crate::triad::Triad;

/// External strategy towards suffering
#[derive(Clone, Copy, PartialEq, Eq)]
#[derive(enum_display::EnumDisplay)]
pub enum Action
{
    #[display("I can change it")]
    Assertive,
    #[display("I can tolerate it")]
    Compliant,
    #[display("I can avoid it")]
    Withdrawn
}

impl Triad for Action
{
    fn expression(&self) -> &'static str
    {
        match self
        {
            Self::Assertive => "I can change it",
            Self::Compliant => "I can tolerate it",
            Self::Withdrawn => "I can avoid it"
        }
    }

    fn reflection(&self) -> &'static str
    {
        match self
        {
            Self::Assertive => "you believe you can change it",
            Self::Compliant => "you believe you can tolerate it",
            Self::Withdrawn => "you believe you can avoid it"
        }
    }
}