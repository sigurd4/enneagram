use std::borrow::Borrow;

use crate::{config::{Config, Fallback}, enneatype::Enneatype, triad::Triad};

#[derive(Clone, Default)]
pub struct Enneagram
{
    edges: Vec<Vec<Enneatype>>
}

impl Enneagram
{
    pub fn all() -> Self
    {
        Self {
            edges: vec![Enneatype::all().to_vec()]
        }
    }

    pub fn paths(&self) -> Vec<Vec<Enneatype>>
    {
        let mut paths = Vec::<Vec<Enneatype>>::new();
        for edges in self.edges.iter()
        {
            for edge in edges.iter()
            {
                if paths.iter().any(|path| path.contains(edge))
                {
                    continue;
                }
                let path = edge.path().collect::<Vec<_>>();
                if path.iter().all(|node| edges.contains(node))
                {
                    paths.push(path);
                }
            }
        }
        paths
    }

    pub fn push_edges(&mut self, edges: impl IntoIterator<Item = Enneatype>)
    {
        self.edges.push(edges.into_iter().collect());
    }
    pub fn is_empty(&self) -> bool
    {
        self.edges.is_empty()
    }
    pub fn edges(&self) -> impl Iterator<Item = &'_ [Enneatype]>
    {
        self.edges.iter()
            .map(Borrow::borrow)
    }

    pub fn lines(&self, config: &Config, fallback: &Fallback) -> Vec<[Enneatype; 2]>
    {
        let mut lines = core::iter::empty()
            .chain(if config.show(fallback).path_lines(fallback) { Some(self.path_lines()) } else { None }.into_iter().flatten())
            .chain(
                if config.show(fallback).boundary_lines(fallback) { Some(self.boundary_lines()) } else { None }
                    .into_iter()
                    .flatten()
            )
            .chain(if config.show(fallback).pivot_lines(fallback) { Some(self.pivot_lines()) } else { None }.into_iter().flatten())
            .chain(if config.show(fallback).triad_lines(fallback) { Some(self.triad_lines()) } else { None }.into_iter().flatten())
            .collect::<Vec<_>>();
        lines.dedup_by(|a, b| crate::line::equals(a, b));
        lines
    }

    pub fn path_lines(&self) -> impl Iterator<Item = [Enneatype; 2]>
    {
        self.paths().into_iter().flat_map(crate::path::lines)
    }

    pub fn boundary_lines(&self) -> impl Iterator<Item = [Enneatype; 2]>
    {
        crate::path::lines(*Enneatype::all()).filter(|line| self.edges.iter().any(|bucket| line.iter().all(|link| bucket.contains(link))))
    }

    pub fn pivot_lines(&self) -> impl Iterator<Item = [Enneatype; 2]>
    {
        Enneatype::all()
            .iter()
            .flat_map(|edge| edge.pivot().lines())
            .filter(|line| self.edges.iter().any(|bucket| line.iter().all(|link| bucket.contains(link))))
    }

    pub fn triad_lines(&self) -> impl Iterator<Item = [Enneatype; 2]>
    {
        self.triads().flat_map(|triad| triad.lines())
    }

    pub fn triads(&self) -> impl Iterator<Item = Box<dyn Triad>>
    {
        crate::triad::all().into_iter().filter(|traid| {
            let triad_edges = traid.edges();

            self.edges.iter().any(|bucket| bucket.iter().all(|edge| triad_edges.contains(edge)))
        })
    }
}

#[cfg(test)]
mod test
{
    use crate::Enneagram;

    #[test]
    fn test_paths()
    {
        let paths = Enneagram::all().paths();

        println!("{paths:?}")
    }
}
