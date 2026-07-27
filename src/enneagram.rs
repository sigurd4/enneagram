use std::borrow::Borrow;

use crate::{config::{Fallback, Config}, enneatype::Enneatype, triad::Triad};

#[derive(Clone, Default)]
pub struct Enneagram
{
    edges: Vec<Vec<Enneatype>>,
    config: Config,
    fallback: Fallback
}

impl Enneagram
{
    pub fn all(original: Enneagram) -> Self
    {
        Self {
            edges: vec![Enneatype::all().to_vec()],
            ..original
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
    pub fn config(&self) -> &Config
    {
        &self.config
    }
    pub fn fallback(&self) -> &Fallback
    {
        &self.fallback
    }
    pub fn overlay_config(&mut self, config: Config)
    {
        self.fallback.add_fallback(core::mem::replace(&mut self.config, config));
    }
    pub fn overlay_configs(&mut self, configs: impl IntoIterator<Item = Config>)
    {
        for config in configs
        {
            self.overlay_config(config);
        }
    }

    pub fn lines(&self) -> Vec<[Enneatype; 2]>
    {
        let mut lines = core::iter::empty()
            .chain(if self.config.show(&self.fallback).path_lines(&self.fallback) { Some(self.path_lines()) } else { None }.into_iter().flatten())
            .chain(
                if self.config.show(&self.fallback).boundary_lines(&self.fallback) { Some(self.boundary_lines()) } else { None }
                    .into_iter()
                    .flatten()
            )
            .chain(if self.config.show(&self.fallback).pivot_lines(&self.fallback) { Some(self.pivot_lines()) } else { None }.into_iter().flatten())
            .chain(if self.config.show(&self.fallback).triad_lines(&self.fallback) { Some(self.triad_lines()) } else { None }.into_iter().flatten())
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
    use crate::{config::Fallback, Enneagram};

    #[test]
    fn test_paths()
    {
        let paths = Enneagram::all(Enneagram {
            edges: Vec::new(),
            config: Default::default(),
            fallback: Fallback::default()
        })
        .paths();
        println!("{paths:?}")
    }
}
