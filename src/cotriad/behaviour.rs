use crate::{cotriad::Cotriad, triad::{Action, Fault, Triad}};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Behaviour
{
    introverted: Fault,
    extroverted: Action,
}

impl Cotriad for Behaviour
{
    fn question(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
    {
        write!(f, "{} and {}", self.extroverted.expression(), self.introverted.expression())
    }
    fn answer(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
    {
        write!(f, "{} and {}", self.extroverted.reflection(), self.introverted.reflection())
    }
}