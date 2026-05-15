use std::sync::LazyLock;

use enneagram::{Enneagram, Ngram, error::EnneatypeOutOfRange};

use crate::Schizotype;

#[derive(Clone, Copy)]
pub struct Schizogram
{

}

impl Ngram for Schizogram
{
    type Edge = Schizotype;
    type Edges = [Self::Edge; 9];
    type EdgeOutOfRangeError = EnneatypeOutOfRange;

    fn all_edges() -> &'static Self::Edges 
    {
        static EDGES: LazyLock<[Schizotype; 9]> = LazyLock::new(enneagram::util::all_enneatypes::<Schizogram>);

        &EDGES
    }
}

impl Enneagram for Schizogram
{
    
}