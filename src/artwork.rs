use core::f32::consts::PI;

use ratatui::{Terminal, layout::Rect, prelude::Backend};
use ratatui_3d::{Light, Material, Mesh, Rgb, Scene, SceneObject, Transform, Viewport3D, Viewport3DState, math::{Quat, Vec3}};

use crate::{enneagram::Enneagram, enneatype::Enneatype, wireframe::Wireframe};

pub struct Artwork<'a>
{
    pub enneagram: &'a Enneagram
}

impl Artwork<'_>
{
    pub fn draw(&self, terminal: &mut Terminal<impl Backend>)
    {
        // Build a scene
        let mut scene = Scene::new()
            .with_background(Rgb(0, 0, 0));

        const RADIUS: f32 = 1.4;
        const LINE_THICKNESS: f64 = 0.02;
        const DIGIT_RADIUS: f64 = 4.0/3.0;
        const DIGIT_PIXEL_SIZE: f64 = 1.0/27.0;

        let transform = Transform {
            rotation: Quat::from_rotation_x(-PI/9.0).mul_quat(Quat::from_rotation_y(0.0)),
            scale: Vec3 { x: RADIUS, y: RADIUS, z: RADIUS },
            position: Vec3 { x: 0.0, y: -0.15, z: -0.0 }
        };
        let all = Enneagram::all(&self.enneagram);

        for edge in all.edges.iter().flatten()
        {
            let [x, y] = edge.position()
                .map(|p| p*DIGIT_RADIUS);
            for line in crate::path::lines_disconnected(
                    edge.digit()
                        .into_iter()
                        .copied()
                        .map(|pos| pos.map(|u| u as f64*DIGIT_PIXEL_SIZE))
                        .map(|[px, py]| [px + x, py + y])
                )
            {
                scene.add_object(
                    SceneObject::new(
                        Wireframe::from_line(line, LINE_THICKNESS)
                        .map(|[x, y]| [x, y, 0.0])
                        .mesh()
                    )
                    .with_material(Material::default().with_color(if self.enneagram.edges.iter().flatten().any(|e| e == edge)
                    {
                        Rgb(255, 255, 255/2)
                    }
                    else
                    {
                        Rgb(255, 0, 0)
                    }))
                    .with_transform(transform),
                );
            }
        }

        let dyed_lines = Wireframe::from_lines(self.enneagram.lines())
            .map(|edge| edge.position())
            .into_lines()
            .collect::<Vec<_>>();

        for line in Wireframe::from_lines(all.lines())
            .map(|edge| edge.position())
            .with_lines(dyed_lines.iter().copied())
            .into_lines()
        {
            scene.add_object(
                SceneObject::new(
                    Wireframe::from_line(line, LINE_THICKNESS)
                        .map(|[x, y]| [x, y, 0.0])
                        .mesh()
                )
                .with_material(Material::default().with_color(if dyed_lines.iter()
                        .any(|dyed_line| crate::line::equals(dyed_line, &line))
                    {
                        Rgb(255, 255, 255/2)
                    }
                    else
                    {
                        Rgb(255, 0, 0)
                    }))
                .with_transform(transform),
            );
        }

        scene.add_object(
            SceneObject::new(
                Wireframe::from_lines(all.path_lines())
                    .map(|edge| edge.position())
                    .extrude(-1.0)
                    .mesh()
            )
            .with_material(Material::default().with_color(Rgb(255, 255, 255)))
            .with_transform(transform),
        );
        
        scene.add_light(Light::ambient(Rgb(255, 255, 255), 1.0));
        scene.add_light(Light::directional(Vec3::new(0.2, 0.2, 1.0), Rgb(255, 255, 255)));

        // Render as a ratatui widget
        let mut state = Viewport3DState::default();
        let max_height = terminal.get_frame().area().height;
        let max_width = terminal.get_frame().area().width;
        let (height, width) = if max_height*8 < max_width*3
        {
            (max_height, max_height*8/3)
        }
        else
        {
            (max_width*3/8, max_width)
        };
        let area = Rect {
            x: 0,
            y: 0,
            width,
            height
        };
        let (Ok(_) | Err(_)) = terminal.draw(|f| f.render_stateful_widget(Viewport3D::new(&scene), area, &mut state));
    }
}

#[cfg(test)]
mod test
{
    use crate::{artwork::Artwork, enneagram::Enneagram};

    #[test]
    fn test_graphics()
    {
        let artwork = Artwork {
            enneagram: &Enneagram::all(&Enneagram {
                edges: vec![],
                show_path_lines: false,
                show_boundary_lines: false,
                show_pivot_lines: false,
                show_triad_lines: false
            })
        };

        let mut terminal = ratatui::init();
        artwork.draw(&mut terminal);
    }
}