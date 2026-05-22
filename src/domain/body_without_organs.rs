use core::{any::Any, ops::Add};

use crate::{config::{DomainConfig, TriadsConfig}, domain::Domain, triad::{Fault, Means, Triad}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BodyWithoutOrgans
{
    pub introverted: Fault,
    pub extroverted: Means,
}

impl BodyWithoutOrgans
{
    pub fn all() -> [BodyWithoutOrgans; 9]
    {
        use {Fault::*, Means::*};

        [
            Positive + Assertive, Competent + Assertive, Reactive + Assertive,
            Positive + Compliant, Competent + Compliant, Reactive + Compliant,
            Positive + Withdrawn, Competent + Withdrawn, Reactive + Withdrawn
        ]
    }

    pub fn kind<'a>(config: &'a DomainConfig) -> &'a str
    {
        config.body_without_organs.as_str()
    }
}

impl Add<Means> for Fault
{
    type Output = BodyWithoutOrgans;

    fn add(self, rhs: Means) -> Self::Output
    {
        BodyWithoutOrgans {
            introverted: self,
            extroverted: rhs
        }
    }
}
impl Add<Fault> for Means
{
    type Output = BodyWithoutOrgans;

    fn add(self, rhs: Fault) -> Self::Output
    {
        BodyWithoutOrgans {
            introverted: rhs,
            extroverted: self
        }
    }
}

impl Domain for BodyWithoutOrgans
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
        &self.extroverted
    }
    fn subconscious(&self) -> &dyn Triad
    {
        &self.introverted
    }
    fn question(&self, f: &mut core::fmt::Formatter<'_>, config: &TriadsConfig) -> core::fmt::Result
    {
        write!(f, "{} and {}", self.extroverted.config(config).expression, self.introverted.config(config).expression)
    }
    fn trivial(&self, f: &mut core::fmt::Formatter<'_>, config: &TriadsConfig) -> core::fmt::Result
    {
        write!(f, "{} and {}", self.extroverted.config(config).reflection, self.introverted.config(config).reflection)
    }
}