use crate::{cotriad::Cotriad, triad::{Action, Frame, Triad}};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct InternalDissonance
{
    thesis: Frame,
    anti_thesis: Action
}

impl Cotriad for InternalDissonance
{
    fn question(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
    {
        write!(f, "{}, but {}", self.thesis.expression(), self.anti_thesis.expression())
    }
    fn answer(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
    {
        write!(f, "{}, so {}", self.thesis.reflection(), self.anti_thesis.reflection())
    }
}