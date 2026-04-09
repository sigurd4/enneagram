moddef::moddef!(
    flat(pub) mod {
        blame,
        humonculus,
        need,
        strategy
    }
);

trait Triad: Sized
{
    fn i(&self) -> &'static str;
    fn you(&self) -> &'static str;
}