use crate::{enneatype::Enneatype, triad::Triad};

pub struct Enneagram
{
    pub edges: Vec<Vec<Enneatype>>,
    pub show_path_lines: bool,
    pub show_boundary_lines: bool,
    pub show_pivot_lines: bool,
    pub show_triad_lines: bool,
}

impl Enneagram
{
    pub fn all(original: &Enneagram) -> Self
    {
        Self {
            edges: vec![
                Enneatype::all()
                    .to_vec()
            ],
            ..*original
        }
    }
    
    pub fn paths(&self) -> Vec<Vec<Enneatype>>
    {
        let mut paths = Vec::<Vec<Enneatype>>::new();
        for edges in self.edges.iter()
        {
            for edge in edges.iter()
            {
                if paths.iter()
                    .any(|path| path.contains(&edge))
                {
                    continue
                }
                let path = edge.path()
                    .collect::<Vec<_>>();
                if path.iter()
                    .all(|node| edges.contains(node))
                {
                    paths.push(path);
                }
            }
        }
        paths
    }

    pub fn lines(&self) -> Vec<[Enneatype; 2]>
    {
        let mut lines = core::iter::empty()
            .chain(if self.show_path_lines {Some(self.path_lines())} else {None}.into_iter().flatten())
            .chain(if self.show_boundary_lines {Some(self.boundary_lines())} else {None}.into_iter().flatten())
            .chain(if self.show_pivot_lines {Some(self.pivot_lines())} else {None}.into_iter().flatten())
            .chain(if self.show_triad_lines {Some(self.triad_lines())} else {None}.into_iter().flatten())
            .collect::<Vec<_>>();
        lines.dedup_by(|a, b| crate::line::equals(a, b));
        lines
    }

    pub fn path_lines(&self) -> impl Iterator<Item = [Enneatype; 2]>
    {
        self.paths()
            .into_iter()
            .flat_map(|path| crate::path::lines(path))
    }

    pub fn boundary_lines(&self) -> impl Iterator<Item = [Enneatype; 2]>
    {
        crate::path::lines(Enneatype::all())
            .filter(|line| self.edges.iter()
                .any(|bucket| line.iter()
                    .all(|link| bucket.contains(link))
                )
            )
    }

    pub fn pivot_lines(&self) -> impl Iterator<Item = [Enneatype; 2]>
    {
        Enneatype::all()
            .into_iter()
            .flat_map(|edge| edge.pivot().lines())
            .filter(|line| self.edges.iter()
                .any(|bucket| line.iter()
                    .all(|link| bucket.contains(link))
                )
            )
    }

    pub fn triad_lines(&self) -> impl Iterator<Item = [Enneatype; 2]>
    {
        self.triads()
            .flat_map(|triad| triad.lines())
    }

    pub fn triads(&self) -> impl Iterator<Item = Box<dyn Triad>>
    {
        crate::triad::all()
            .into_iter()
            .filter(|traid| {
                let triad_edges = traid.edges();

                self.edges.iter()
                    .any(|bucket| bucket.iter()
                        .all(|edge| triad_edges.contains(edge))
                    )
            })
    }
}

#[cfg(test)]
mod test
{
    use crate::enneagram::Enneagram;

    #[test]
    fn test_paths()
    {
        let paths = Enneagram::all(&Enneagram {
            edges: vec![],
            show_path_lines: false,
            show_boundary_lines: false,
            show_pivot_lines: false,
            show_triad_lines: false
        }).paths();
        println!("{paths:?}")
    }
}