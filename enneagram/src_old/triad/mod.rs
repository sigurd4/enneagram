
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(enum_display::EnumDisplay)]
pub enum Triad
{
    #[display("Internal strategy for one's (meta-)suffering/\"who to blame?\"")]
    Fault(Fault),
    #[display("Homonculus of the self/internalization of self/frame of judgement/meta-objective/\"Who am i?\"")]
    Frame(Frame),
    #[display("Need/object of desire/\"what hole do you have in your soul?\"")]
    Need(Need),
    #[display("External strategy towards suffering / means")]
    Means(Means)
}