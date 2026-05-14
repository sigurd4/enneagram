use core::{error::Error, fmt::Debug, num::NonZeroU8};

use array_trait::{Array, AsSlice};

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

pub fn zeroable_edge_try_from<E, EE>(number: u8, all_edges: &EE, out_of_range: ) -> Result<E, E::Error>
where
    EE: AsSlice
{
    G::all_edges()
        .get(i as usize)
        .copied()
}

pub fn nonzero_edge_try_from<E, EE>(number: u8, all_edges: &EE) -> Result<E, E::Error>
where
    EE: AsSlice
{
    let i = NonZeroU8::try_from(number)
        .map_err(EnneatypeOutOfRange::CantBeZero)?
        .get() - 1;

    G::all_edges()
        .get(i as usize)
        .copied()
        .ok_or(EnneatypeOutOfRange::OutOfRange)
}

pub mod error
{
    pub trait EdgeOutOfRangeError: Error + Clone + Copy
    {

    }
}