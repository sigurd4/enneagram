use core::{any::Any, ops::Add};

use crate::{domain::Domain, triad::{Means, Need, Triad}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExternalConflict
{
    pub thesis: Need,
    pub anti_thesis: Means
}

impl ExternalConflict
{
    pub fn all() -> [ExternalConflict; 9]
    {
        use {Need::*, Means::*};

        [
            Attachment + Assertive, Frustration + Assertive, Rejection + Assertive,
            Attachment + Compliant, Frustration + Compliant, Rejection + Compliant,
            Attachment + Withdrawn, Frustration + Withdrawn, Rejection + Withdrawn
        ]
    }

    pub fn kind() -> &'static str
    {
        "external conflict"
    }
}

impl Add<Means> for Need
{
    type Output = ExternalConflict;

    fn add(self, rhs: Means) -> Self::Output
    {
        ExternalConflict {
            thesis: self,
            anti_thesis: rhs
        }
    }
}
impl Add<Need> for Means
{
    type Output = ExternalConflict;

    fn add(self, rhs: Need) -> Self::Output
    {
        ExternalConflict {
            thesis: rhs,
            anti_thesis: self
        }
    }
}

impl Domain for ExternalConflict
{
    fn as_any(&self) -> &dyn Any
    {
        self
    }
    fn equals(&self, other: &dyn Domain) -> bool
    {
        other.as_any().downcast_ref().is_some_and(|other| self == other)
    }
    
    fn kind(&self) -> &'static str
    {
        Self::kind()
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