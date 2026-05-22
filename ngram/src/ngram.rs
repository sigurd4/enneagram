use array_trait::Array;

use crate::{Edge, error::EdgeOutOfRangeError};

pub trait Group
{

}

pub trait Ngram: Sized
{
    type Edge: Edge<Self>;
    type Edges: Array<Elem = Self::Edge>;
    type EdgeOutOfRangeError: EdgeOutOfRangeError;

    type Group: Group;
    type Groups: Array<Elem = Self::Group>;

    fn all_edges() -> &'static Self::Edges;
}

#[cfg(test)]
mod test
{
    #[test]
    fn test1()
    {
        let mut triads = vec![];

        let digits = [1, 2, 3, 4, 5, 6, 7, 8, 9];

        let mut triad = digits;
        
        for t in [
            [2, 5, 5, 5, 8, 8, 8, 2, 2],
            [2, 5, 2, 5, 8, 5, 8, 2, 8],
            [4, 8, 6, 4, 8, 6, 4, 8, 6],
            [5, 1, 4, 0, 5, 2, 7, 3, 0]
        ]
        {
            let prev = triad;
            for (i, k) in t.into_iter()
                .enumerate()
            {
                triad[(i + k) % digits.len()]
                    = prev[i]
            }

            triads.push(triad);

            println!("{triad:?}");
        }

        assert_eq!(
            triads,
            &[
                [8, 9, 1, 5, 6, 7, 2, 3, 4],
                [3, 7, 8, 6, 1, 2, 9, 4, 5],
                [7, 9, 2, 1, 3, 5, 4, 6, 8],
                [3, 6, 9, 1, 4, 7, 2, 5, 8]
            ]
        )

        // 891 567 234

        // 378 612 945

        // 792 135 468

        // 369 147 258
    }
}