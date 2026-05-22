use core::any::Any;

use crate::{config::{TriadConfig, TriadsConfig}, enneatype::Enneatype, triad::Triad};

/// Need/object of desire/"what hole do you have in your soul?"
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Need
{
    Attachment,
    Frustration,
    Rejection
}
impl Need
{
    pub const fn all() -> [Self; 3]
    {
        [Need::Attachment, Need::Frustration, Need::Rejection]
    }
}

impl Triad for Need
{
    fn as_any(&self) -> &dyn Any
    {
        self
    }
    fn equals(&self, other: &dyn Triad) -> bool
    {
        other.as_any().downcast_ref().is_some_and(|other| self == other)
    }

    fn edges(&self) -> &'static [Enneatype; 3]
    {
        match self
        {
            Need::Attachment => &[Enneatype::Repression, Enneatype::Paranoia, Enneatype::Rest], // 369
            Need::Frustration => &[Enneatype::Recovery, Enneatype::Rejection, Enneatype::Disorganization], // 147
            Need::Rejection => &[Enneatype::Association, Enneatype::Catatonia, Enneatype::Action], // 258
        }
    }
    fn config<'a>(&self, config: &'a TriadsConfig) -> &'a TriadConfig
    {
        match self
        {
            Need::Attachment => &config.attachment,
            Need::Frustration => &config.frustration,
            Need::Rejection => &config.rejection
        }
    }
}