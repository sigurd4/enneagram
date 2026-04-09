use crate::{cotriad::Cotriad, triad::{Fault, Need, Triad}};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ExternalDissonance
{
    anti_thesis: Fault,
    thesis: Need
}

impl Cotriad for ExternalDissonance
{
    fn question(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
    {
        write!(f, "{}, but {}", self.anti_thesis.expression(), self.thesis.expression())
    }
    fn answer(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
    {
        write!(f, "{}, because {}", self.anti_thesis.reflection(), self.thesis.reflection())
    }
}