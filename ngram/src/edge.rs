use core::fmt::Debug;

use crate::Ngram;

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