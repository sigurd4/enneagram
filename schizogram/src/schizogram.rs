use enneagram::{Enneagram, Ngram, error::EnneatypeOutOfRange};

use crate::Schizotype;

pub struct Schizogram
{

}

impl Ngram for Schizogram
{
    type Edge = Schizotype;
    type Edges = [Self::Edge; 9];
    type EdgeOutOfRangeError = EnneatypeOutOfRange;
}

impl Enneagram for Schizogram
{
    
}