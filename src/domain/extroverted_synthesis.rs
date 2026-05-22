use core::{any::Any, ops::Add};

use crate::{config::{DomainConfig, TriadsConfig}, domain::Domain, triad::{Means, Need, Triad}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExternalSynthesis
{
    pub thesis: Need,
    pub anti_thesis: Means
}

impl ExternalSynthesis
{
    pub fn all() -> [ExternalSynthesis; 9]
    {
        use {Need::*, Means::*};

        [
            Attachment + Assertive, Frustration + Assertive, Rejection + Assertive,
            Attachment + Compliant, Frustration + Compliant, Rejection + Compliant,
            Attachment + Withdrawn, Frustration + Withdrawn, Rejection + Withdrawn
        ]
    }

    pub fn kind<'a>(config: &'a DomainConfig) -> &'a str
    {
        config.extroverted_synthesis.as_str()
    }
}

impl Add<Means> for Need
{
    type Output = ExternalSynthesis;

    fn add(self, rhs: Means) -> Self::Output
    {
        ExternalSynthesis {
            thesis: self,
            anti_thesis: rhs
        }
    }
}
impl Add<Need> for Means
{
    type Output = ExternalSynthesis;

    fn add(self, rhs: Need) -> Self::Output
    {
        ExternalSynthesis {
            thesis: rhs,
            anti_thesis: self
        }
    }
}

impl Domain for ExternalSynthesis
{
    fn as_any(&self) -> &dyn Any
    {
        self
    }
    fn equals(&self, other: &dyn Domain) -> bool
    {
        other.as_any().downcast_ref().is_some_and(|other| self == other)
    }
    
    fn kind<'a>(&self, config: &'a DomainConfig) -> &'a str
    {
        Self::kind(config)
    }
    fn conscious(&self) -> &dyn Triad
    {
        &self.anti_thesis
    }
    fn subconscious(&self) -> &dyn Triad
    {
        &self.thesis
    }
    fn question(&self, f: &mut core::fmt::Formatter<'_>, config: &TriadsConfig) -> core::fmt::Result
    {
        write!(f, "{}, but {}", self.anti_thesis.config(config).expression, self.thesis.config(config).expression)
    }
    fn trivial(&self, f: &mut core::fmt::Formatter<'_>, config: &TriadsConfig) -> core::fmt::Result
    {
        write!(f, "{}, because {}", self.anti_thesis.config(config).reflection, self.thesis.config(config).reflection)
    }
}