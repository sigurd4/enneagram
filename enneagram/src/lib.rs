use core::num::NonZeroU8;

pub use ngram::{Edge, Ngram};

use crate::error::EnneatypeOutOfRange;

pub trait Enneagram: Ngram<EdgeOutOfRangeError = error::EnneatypeOutOfRange, Edges = [<Self as Ngram>::Edge; 9]>
{

}

pub trait Enneatype<G>: Edge<G>
where
    G: Enneagram<Edge = Self>
{

}

pub mod error
{
    use core::num::TryFromIntError;

    #[derive(Clone, Copy, Debug, thiserror::Error)]
    pub enum EnneatypeOutOfRange
    {
        #[error("There is no enneatype with the number 0.")]
        CantBeZero(TryFromIntError),
        #[error("Enneagram numbers must be within the range of 1-9.")]
        OutOfRange
    }
}