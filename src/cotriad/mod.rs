use crate::triad::Triad;

moddef::moddef!(
    flat(pub) mod {
        external_dissonance,
        external_conflict,
        behaviour,
        suffering,
        internal_conflict,
        internal_dissonance
    }
);

trait Cotriad
{
    fn question(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result;
    fn answer(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result;
}