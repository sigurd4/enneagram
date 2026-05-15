use enneagram::{Edge, Enneatype, Ngram, error::EnneatypeOutOfRange};

use crate::Schizogram;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[derive(enum_display::EnumDisplay)]
pub enum Schizotype
{
    #[display("Recovery/Gradient")]
    Recovery = 1,
    #[display("Association/Superego")]
    Association = 2,
    #[display("Repression/Ego")]
    Repression = 3,
    #[display("Rejection/Id")]
    Rejection = 4,
    #[display("Catatonia")]
    Catatonia = 5,
    #[display("Paranoia")]
    Paranoia = 6,
    #[display("Disorganization")]
    Disorganization = 7,
    #[display("Action/Flow")]
    Action = 8,
    #[display("Rest/Equilibrium")]
    Rest = 9
}

impl TryFrom<u8> for Schizotype
{
    type Error = EnneatypeOutOfRange;

    fn try_from(number: u8) -> Result<Self, Self::Error>
    {
        enneagram::util::enneatype_try_from(number, Schizogram::all_edges())
    }
}

impl Edge<Schizogram> for Schizotype
{
    
}

impl Enneatype<Schizogram> for Schizotype
{

}