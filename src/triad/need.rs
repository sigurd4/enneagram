use crate::triad::Triad;

/// Need/object of desire/"what hole do you have in your soul?"
#[derive(Clone, Copy, PartialEq, Eq)]
#[derive(enum_display::EnumDisplay)]
pub enum Need
{
    #[display("I need freedom")]
    Attachment,
    #[display("I need control")]
    Frustration,
    #[display("I need affection")]
    Rejection
}

impl Triad for Need
{
    fn i(&self) -> &'static str
    {
        match self
        {
            Need::Attachment => "I need freedom",
            Need::Frustration => "I need control",
            Need::Rejection => "I need affection",
        }
    }

    fn you(&self) -> &'static str
    {
        match self
        {
            Need::Attachment => "you crave freedom",
            Need::Frustration => "you crave control",
            Need::Rejection => "you crave affection"
        }
    }
}