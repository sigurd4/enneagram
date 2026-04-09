use crate::triad::Triad;

/// Internal strategy for one's (meta-)suffering/"who to blame?"
#[derive(Clone, Copy, PartialEq, Eq)]
#[derive(enum_display::EnumDisplay)]
pub enum Fault
{
    #[display("everything is fine")]
    Positive,
    #[display("I take responsibility")]
    Competent,
    #[display("it's their fault")]
    Reactive
}

impl Triad for Fault
{
    fn expression(&self) -> &'static str
    {
        match self
        {
            Fault::Positive => "everything is fine",
            Fault::Competent => "I take responsibility",
            Fault::Reactive => "it's their fault",
        }
    }

    fn reflection(&self) -> &'static str
    {
        match self
        {
            Fault::Positive => "you delude yourself into thinking that everything is fine",
            Fault::Competent => "you hold yourself responsible",
            Fault::Reactive => "you blame others"
        }
    }
}