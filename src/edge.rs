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