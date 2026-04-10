#![feature(iter_next_chunk)]

use crate::{domain::{Domain, InternalConflict}, triad::{Fault, Frame}};

moddef::moddef!(
    mod {
        edge,
        domain,
        triad
    }
);

fn main()
{
    let cotriad = InternalConflict {
        thesis: Frame::Gut,
        anti_thesis: Fault::Competent
    };

    let a = std::fmt::from_fn(|f| cotriad.trivial(f));
    println!("{}", a);
}


#[cfg(test)]
mod test
{
    #[test]
    fn it_works()
    {
        
    }
}