use core::f32::consts::PI;

use ratatui::{Terminal, layout::Rect, prelude::Backend};
use ratatui_3d::{Light, Material, Scene, SceneObject, Transform, Viewport3D, Viewport3DState, math::{Quat, Vec3}};

use enneagram::{Enneagram, line, path, wireframe::Wireframe};

pub struct Artwork
{
    pub enneagram: Enneagram
}

impl Artwork
{
    pub fn draw(&self, terminal: &mut Terminal<impl Backend>)
    {
        // Build a scene
        let mut scene = Scene::new()
            .with_background(self.enneagram.config().color(&self.enneagram.fallback()).sky(&self.enneagram.fallback()));

        const RADIUS: f32 = 1.4;
        const LINE_THICKNESS: f64 = 0.02;
        const DIGIT_RADIUS: f64 = 4.0/3.0;
        const DIGIT_PIXEL_SIZE: f64 = 1.0/27.0;

        let transform = Transform {
            rotation: Quat::from_rotation_x(-PI/9.0).mul_quat(Quat::from_rotation_y(0.0)),
            scale: Vec3 { x: RADIUS, y: RADIUS, z: RADIUS },
            position: Vec3 { x: 0.0, y: -0.15, z: -0.0 }
        };
        let all = Enneagram::all(self.enneagram.clone());

        for edge in all.edges().flatten()
        {
            let [x, y] = edge.position()
                .map(|p| p*DIGIT_RADIUS);
            for line in path::lines_disconnected(
                    edge.config(self.enneagram.config(), &self.enneagram.fallback())
                        .digit
                        .iter()
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
                    .with_material(Material::default()
                        .with_color(self.enneagram.config().color(&self.enneagram.fallback())
                            .line(
                                &self.enneagram.fallback(),
                                self.enneagram.edges()
                                    .flatten()
                                    .any(|e| e == edge)
                            )
                        )
                    )
                    .with_transform(transform)
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
                .with_material(Material::default()
                    .with_color(self.enneagram.config().color(&self.enneagram.fallback())
                        .line(
                            &self.enneagram.fallback(),
                            dyed_lines.iter()
                                .any(|dyed_line| line::equals(dyed_line, &line))
                        )
                    )
                )
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
            .with_material(Material::default().with_color(self.enneagram.config().color(&self.enneagram.fallback()).surface(&self.enneagram.fallback())))
            .with_transform(transform),
        );
        
        scene.add_light(Light::ambient(self.enneagram.config().color(&self.enneagram.fallback()).glare(&self.enneagram.fallback()), 1.0));
        scene.add_light(Light::directional(Vec3::new(-0.2, -0.2, 1.0),  self.enneagram.config().color(&self.enneagram.fallback()).sun(&self.enneagram.fallback())));
        scene.add_light(Light::directional(Vec3::new(0.2, 0.2, 1.0),  self.enneagram.config().color(&self.enneagram.fallback()).shine(&self.enneagram.fallback())));

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
        let _ = terminal.draw(|f| f.render_stateful_widget(Viewport3D::new(&scene), area, &mut state))
            .expect("Rendering failed.");
        println!("\x1b[0m")
    }
}

#[cfg(test)]
mod test
{
    use crate::{artwork::Artwork, Enneagram};

    #[test]
    #[ignore] // It fucks up the keyboard input, because i'm not fluent in ratatui
    fn test_graphics()
    {
        let artwork = Artwork {
            enneagram: Enneagram::all(Enneagram::default())
        };

        let mut terminal = ratatui::init();
        artwork.draw(&mut terminal);
    }
}
