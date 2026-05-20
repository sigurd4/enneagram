use array_trait::Array;

use crate::{Edge, error::EdgeOutOfRangeError};

pub trait Ngram: Sized
{
    type Edge: Edge<Self>;
    type Edges: Array<Elem = Self::Edge>;
    type EdgeOutOfRangeError: EdgeOutOfRangeError;

    fn all_edges() -> &'static Self::Edges;
}