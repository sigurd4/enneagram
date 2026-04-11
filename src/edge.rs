use core::f64::consts::TAU;

use crate::triad::Triad;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(enum_display::EnumDisplay)]
pub enum Edge
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

impl Edge
{
    pub fn number(&self) -> u8
    {
        let number = *self as u8;
        assert!(number >= 1 && number <= 9, "Enneagram numbers must be within the range of 1-9");
        number
    }

    pub fn triads(&self) -> [Box<dyn Triad>; 4]
    {
        crate::triad::all()
            .into_iter()
            .filter(|traid| traid.edges().contains(&self))
            .collect::<Vec<_>>()
            .try_into()
            .expect("Each personality must consist of exactly 4 triads.")
    }

    pub fn angle(&self) -> f64
    {
        let number = self.number();
        number as f64/10.0*TAU
    }

    pub fn position(&self) -> (f64, f64)
    {
        let angle = self.angle();
        let (sine, cosine) = angle.sin_cos();
        (sine, cosine)
    }
}