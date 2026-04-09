use crate::{cotriad::Cotriad, triad::{Action, Need, Triad}};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ExternalConflict
{
    thesis: Need,
    anti_thesis: Action
}

impl Cotriad for ExternalConflict
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