use crate::{cotriad::Cotriad, triad::{Fault, Need, Triad}};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Suffering
{
    introverted: Fault,
    extroverted: Need
}

impl Cotriad for Suffering
{
    fn question(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
    {
        write!(f, "{} and {}", self.introverted.expression(), self.extroverted.expression())
    }
    fn answer(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
    {
        write!(f, "{} and {}", self.introverted.reflection(), self.extroverted.reflection())
    }
}