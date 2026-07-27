use core::{any::Any, ops::Add};

use crate::{config::{Fallback, Property, DomainConfig, TriadsConfig}, domain::Domain, triad::{Means, Need, Triad}};

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

    pub fn kind<'a>(config: &'a (impl Property<DomainConfig> + ?Sized), fallback: &'a Fallback) -> &'a str
    {
        config.property(fallback).extroverted_synthesis(fallback)
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
    
    fn kind<'a>(&self, config: &'a dyn Property<DomainConfig>, fallback: &'a Fallback) -> &'a str
    {
        Self::kind(config, fallback)
    }
    fn conscious(&self) -> &dyn Triad
    {
        &self.anti_thesis
    }
    fn subconscious(&self) -> &dyn Triad
    {
        &self.thesis
    }
    fn question(&self, f: &mut core::fmt::Formatter<'_>, config: &dyn Property<TriadsConfig>, fallback: &Fallback) -> core::fmt::Result
    {
        write!(f, "{}, but {}", self.anti_thesis.config(config, fallback).expression, self.thesis.config(config, fallback).expression)
    }
    fn trivial(&self, f: &mut core::fmt::Formatter<'_>, config: &dyn Property<TriadsConfig>, fallback: &Fallback) -> core::fmt::Result
    {
        write!(f, "{}, because {}", self.anti_thesis.config(config, fallback).reflection, self.thesis.config(config, fallback).reflection)
    }
}
