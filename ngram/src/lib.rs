use core::fmt::Debug;

use array_trait::Array;

use crate::error::EdgeOutOfRangeError;

pub trait Ngram: Sized
{
    type Edge: Edge<Self>;
    type Edges: Array<Elem = Self::Edge>;
    type EdgeOutOfRangeError: EdgeOutOfRangeError;

    fn all_edges() -> &'static Self::Edges;
}

pub trait Edge<G>: Sized
    + Debug
    + Clone
    + Copy
    + Ord
    + TryFrom<u8, Error = G::EdgeOutOfRangeError>
where
    G: Ngram<Edge = Self>
{
    
}

pub mod error
{
    use core::error::Error;

    pub trait EdgeOutOfRangeError: Error + Clone + Copy
    {

    }
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