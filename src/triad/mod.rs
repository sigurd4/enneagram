moddef::moddef!(
    flat(pub) mod {
        blame,
        humonculus,
        need,
        strategy
    }
);

pub trait Triad: Sized
{
    fn i(&self) -> &'static str;
    fn you(&self) -> &'static str;
}