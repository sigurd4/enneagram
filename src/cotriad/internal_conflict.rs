use crate::{cotriad::Cotriad, triad::{Fault, Frame, Triad}};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct InternalConflict
{
    thesis: Frame,
    anti_thesis: Fault
}

impl Cotriad for InternalConflict
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