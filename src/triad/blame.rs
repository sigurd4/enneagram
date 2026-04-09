use crate::triad::Triad;

/// Internal strategy for one's (meta-)suffering/"who to blame?"
#[derive(Clone, Copy, PartialEq, Eq)]
#[derive(enum_display::EnumDisplay)]
enum Blame
{
    #[display("everything is fine")]
    Positive,
    #[display("I take responsibility")]
    Competent,
    #[display("it's their fault")]
    Reactive
}

impl Triad for Blame
{
    fn i(&self) -> &'static str
    {
        match self
        {
            Blame::Positive => "everything is fine",
            Blame::Competent => "I take responsibility",
            Blame::Reactive => "it's their fault",
        }
    }

    fn you(&self) -> &'static str
    {
        match self
        {
            Blame::Positive => "you delude yourself into thinking that everything is fine",
            Blame::Competent => "you hold yourself responsible",
            Blame::Reactive => "you blame others"
        }
    }
}