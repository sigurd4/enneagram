moddef::moddef!(
    flat(pub) mod {
        fault,
        frame,
        need,
        action
    }
);

pub trait Triad: Sized
{
    fn expression(&self) -> &'static str;
    fn reflection(&self) -> &'static str;
}