#[derive(Clone)]
pub struct Wireframe<P>
{
    points: Vec<P>,
    lines: Vec<[usize; 2]>
}

impl<P> Default for Wireframe<P>
{
    fn default() -> Self
    {
        Self {
            points: Vec::new(),
            lines: Vec::new()
        }
    }
}

impl<P> Wireframe<P>
{
    pub fn new() -> Self
    {
        Self::default()
    }

    pub fn from_path(path: impl IntoIterator<Item = P>) -> Self
    where
        P: Copy + PartialEq
    {
        Self::from_lines(crate::path::lines(path))
    }

    pub fn from_lines(lines: impl IntoIterator<Item = [P; 2]>) -> Self
    where
        P: PartialEq
    {
        Self::new()
            .with_lines(lines)
    }

    pub fn with_lines(mut self, lines: impl IntoIterator<Item = [P; 2]>) -> Self
    where
        P: PartialEq
    {
        self.add_lines(lines);
        self
    }

    pub fn add_lines(&mut self, lines: impl IntoIterator<Item = [P; 2]>)
    where
        P: PartialEq
    {
        self.lines.extend(
            lines.into_iter()
                .map(|line| line.map(|point| {
                    // Add the point if missing, or use exisiting
                    if let Some(index) = self.points.iter()
                        .position(|p| *p == point)
                    {
                        index
                    }
                    else
                    {
                        let index = self.points.len();
                        self.points.push(point);
                        index
                    }
                }))
        );
        self.lines.sort();
        self.lines.dedup_by(|l1, l2| crate::line::equals(l1, l2));
    }

    pub fn map<U>(self, mapper: impl Fn(P) -> U) -> Wireframe<U>
    where
        U: Copy + PartialEq
    {
        let Self { points, lines } = self;

        let mut wireframe = Wireframe {
            points: points.into_iter()
                .map(mapper)
                .collect(),
            lines
        };
        wireframe.dedup();
        wireframe
    }

    pub fn dedup(&mut self)
    where
        P: Copy + PartialEq
    {
        *self = Self::from_lines(core::mem::replace(self, Self::default()).into_lines());
    }

    pub fn into_lines<'a>(self) -> impl Iterator<Item = [P; 2]> + 'a
    where
        P: Copy + 'a
    {
        let Self { points, lines } = self;

        lines.into_iter()
            .map(move |[a, b]| [points[a], points[b]])
    }

    pub fn lines<'a>(&'a self) -> impl Iterator<Item = [P; 2]> + 'a
    where
        P: Copy + 'a
    {
        let Self { points, lines } = self;

        lines.iter()
            .copied()
            .map(move |[a, b]| [points[a], points[b]])
    }

    pub fn corner_indices<'a>(&'a self) -> impl Iterator<Item = [usize; 3]> + 'a
    where
        P: Copy + PartialEq
    {
        // Assemble corners at all lines that share points.
        self.lines.iter()
            .enumerate()
            .flat_map(move |(i, l1 @ [a1, b1])| self.lines[..i].iter()
                .filter_map(move |l2 @ [a2, b2]| {
                    assert!(!crate::line::equals(l1, l2), "Lines cannot overlap exactly");
                    assert!(
                        !crate::line::equals(&[self.points[*a1], self.points[*b1]], &[self.points[*a2], self.points[*b2]]),
                        "Lines cannot overlap"
                    );
                    crate::line::corner(*l1, *l2)
                })
            )
    }

    pub fn corners<'a>(&'a self) -> impl Iterator<Item = [P; 3]> + 'a
    where
        P: Copy + PartialEq
    {
        self.corner_indices()
            .map(|[a, p, b]| [self.points[a], self.points[p], self.points[b]])
    }
}

impl Wireframe<[f64; 3]>
{
    #[cfg(feature = "artwork")]
    pub fn mesh(&self) -> ratatui_3d::Mesh
    {
        use ratatui_3d::{Mesh, Vertex, math::Vec3};

        let vertices = self.corners()
            .map(|[a, p, b]| {

                let as_vec = |[x, y, z]: [f64; 3]| {
                    Vec3::new(x as f32, y as f32, z as f32)
                };
                let [mut a, p, mut b] = [
                    as_vec(a),
                    as_vec(p),
                    as_vec(b)
                ];

                let mut normal = (a - p).cross(b - p);
                if normal.z > 0.0
                {
                    normal = -normal;
                    core::mem::swap(&mut a, &mut b)
                }

                [
                    Vertex::new(a, normal).with_uv(0.0, 0.0),
                    Vertex::new(p, normal).with_uv(0.5, 1.0),
                    Vertex::new(b, normal).with_uv(1.0, 0.0)
                ]
            }).flatten()
            .collect::<Vec<_>>();
        assert_eq!(vertices.len() % 3, 0);
        let indices = (0..vertices.len() as u32).collect();

        Mesh::new(
            vertices,
            indices
        )
    }
}

impl Wireframe<[f64; 2]>
{
    pub fn from_line(line: [[f64; 2]; 2], thickness: f64) -> Self
    {
        let [[ax, ay], [bx, by]] = line;
        let [vx, vy] = [bx - ax, by - ay];
        let v = (vx*vx + vy*vy).sqrt();
        let [wx, wy] = [vy/v*thickness, -vx/v*thickness];
        Wireframe::from_path([
            [ax + wx, ay + wy],
            [bx + wx, by + wy],
            [ax - wx, ay - wy],
            [bx - wx, by - wy]
        ])
    }

    pub fn extrude(mut self, z: f64) -> Wireframe<[f64; 3]>
    {
        self.fragment();
        self.map(|[x, y]| [x, y, z + (1.0 - x*x - y*y).sqrt()])
    }

    pub fn fragmented(mut self) -> Self
    {
        self.fragment();
        self
    }

    pub fn fragment(&mut self) -> bool
    {
        let mut changed = false;
        let mut i = 0;
        'lp1:
        while i < self.lines.len()
        {
            for j in 0..i
            {
                let l1 @ [a1, b1] = self.lines[i];
                let l2 @ [a2, b2] = self.lines[j];

                // Remove duplicate lines
                if crate::line::equals(&l1, &l2)
                {
                    self.lines.remove(i);
                    continue 'lp1
                }

                // Skip lines meeting at a single shared point
                if crate::line::corner(l1, l2).is_some()
                {
                    continue
                }

                let [a1x, a1y] = self.points[a1];
                let [b1x, b1y] = self.points[b1];
                let [a2x, a2y] = self.points[a2];
                let [b2x, b2y] = self.points[b2];
                
                let d1x = b1x - a1x;
                let d1y = b1y - a1y;
                let d2x = b2x - a2x;
                let d2y = b2y - a2y;
                
                let x = (d1x*d2y*a2x - (a2y - a1y)*d1x*d2x - d2x*d1y*a1x)/(d1x*d2y - d2x*d1y);
                let y = (d1x*d2y*a1y + (a2x - a1x)*d1y*d2y - d2x*d1y*a2y)/(d1x*d2y - d2x*d1y);
                
                // Lines intersect? Split them and create new point at intersection.
                if a1x.min(b1x).max(a2x.min(b2x)) < x
                    && x < a1x.max(b1x).min(a2x.max(b2x))
                {
                    changed = true;

                    // Create new point or use existing.
                    let p = if let Some(p) = self.points.iter()
                        .position(|point| *point == [x, y])
                    {
                        p
                    }
                    else
                    {
                        let p = self.points.len();
                        self.points.push([x, y]);
                        p
                    };
                    self.lines[i] = [a1, p];
                    self.lines[j] = [a2, p];
                    self.lines.extend([[p, b1], [p, b2]]);

                    // Go back
                    i = i.min(j);
                    continue 'lp1
                }
            }
            i += 1
        }
        if changed
        {
            self.dedup();
        }
        changed
    }
}