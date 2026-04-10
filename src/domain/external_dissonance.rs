use core::{any::Any, ops::Add};

use crate::{domain::Domain, triad::{Fault, Need, Triad}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExternalDissonance
{
    pub anti_thesis: Fault,
    pub thesis: Need
}

impl ExternalDissonance
{
    pub fn all() -> [ExternalDissonance; 9]
    {
        use {Fault::*, Need::*};

        [
            Positive + Attachment, Competent + Attachment, Reactive + Attachment,
            Positive + Frustration, Competent + Frustration, Reactive + Frustration,
            Positive + Rejection, Competent + Rejection, Reactive + Rejection
        ]
    }

    pub fn kind() -> &'static str
    {
        "external dissonance"
    }
}

impl Add<Need> for Fault
{
    type Output = ExternalDissonance;

    fn add(self, rhs: Need) -> Self::Output
    {
        ExternalDissonance {
            thesis: rhs,
            anti_thesis: self
        }
    }
}
impl Add<Fault> for Need
{
    type Output = ExternalDissonance;

    fn add(self, rhs: Fault) -> Self::Output
    {
        ExternalDissonance {
            thesis: self,
            anti_thesis: rhs
        }
    }
}

impl Domain for ExternalDissonance
{
    fn as_any(&self) -> &dyn Any
    {
        self
    }
    fn equals(&self, other: &dyn Domain) -> bool
    {
        other.as_any().downcast_ref().is_some_and(|other| self == other)
    }
    
    fn conscious(&self) -> &dyn Triad
    {
        &self.anti_thesis
    }
    fn subconscious(&self) -> &dyn Triad
    {
        &self.thesis
    }
    fn question(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
    {
        write!(f, "{}, but {}", self.anti_thesis.expression(), self.thesis.expression())
    }
    fn trivial(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
    {
        write!(f, "{}, because {}", self.anti_thesis.reflection(), self.thesis.reflection())
    }
}