pub use ngram::{Edge, Ngram};

pub trait Enneagram: Ngram<EdgeOutOfRangeError = error::EnneatypeOutOfRange, Edges = [<Self as Ngram>::Edge; 9]>
{

}

pub trait Enneatype<G>: Edge<G>
where
    G: Enneagram<Edge = Self>
{

}

pub mod util
{
    use crate::{Enneagram, error::EnneatypeOutOfRange};

    pub fn all_enneatypes<G>() -> [G::Edge; 9]
    where
        G: Enneagram
    {
        core::array::from_fn(|i| G::Edge::try_from(i as u8 + 1).expect("Enneagram should consist of exactly 9 enneatypes"))
    }

    pub fn enneatype_try_from<E>(number: u8, all_edges: &[E; 9]) -> Result<E, EnneatypeOutOfRange>
    where
        E: Copy
    {
        let i = number as usize;

        all_edges.as_slice()
            .get(i)
            .copied()
            .ok_or(EnneatypeOutOfRange::OutOfRange)
    }
}

pub mod error
{
    use core::num::TryFromIntError;

use ngram::error::EdgeOutOfRangeError;

    #[derive(Clone, Copy, Debug, thiserror::Error)]
    pub enum EnneatypeOutOfRange
    {
        #[error("There is no enneatype with the number 0.")]
        CantBeZero(TryFromIntError),
        #[error("Enneagram numbers must be within the range of 1-9.")]
        OutOfRange
    }

    impl EdgeOutOfRangeError for EnneatypeOutOfRange
    {
        
    }
}