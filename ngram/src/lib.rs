#![feature(f16)]
#![feature(f128)]
#![feature(option_into_flat_iter)]

moddef::moddef!(
    flat(pub) mod {
        corner,
        edge,
        line,
        ngram,
        point,
        points
    }
);

pub mod error
{
    use core::error::Error;

use crate::{Line, Point};

    pub trait EdgeOutOfRangeError: Error + Clone + Copy
    {

    }

    #[derive(Clone, Copy, Debug, thiserror::Error)]
    #[error("Lines do not meet!")]
    pub struct LinesDoNotMeet<P>(pub(crate) Line<P>, pub(crate) Line<P>)
    where
        P: Point;
}

pub mod util
{
    use core::num::{NonZeroU8, TryFromIntError};

use array_trait::AsSlice;

    pub fn zeroable_edge_try_from<E, EE, R>(number: u8, all_edges: &EE, out_of_range: R) -> Result<E, R>
    where
        EE: AsSlice<Elem = E>,
        E: Copy
    {
        let i = number as usize;

        all_edges.as_slice()
            .get(i)
            .copied()
            .ok_or(out_of_range)
    }

    pub fn nonzero_edge_try_from<E, EE, R>(number: u8, all_edges: &EE, cant_be_zero: impl FnOnce(TryFromIntError) -> R, out_of_range: R) -> Result<E, R>
    where
        EE: AsSlice<Elem = E>,
        E: Copy
    {
        let index = NonZeroU8::try_from(number)
            .map_err(cant_be_zero)?
            .get() - 1;

        zeroable_edge_try_from(index, all_edges, out_of_range)
    }

}