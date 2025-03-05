use piston_window::*;
use delaunator::{triangulate, Point as DelPoint};

mod models {
    use std::hash::{Hash, Hasher};
    
    #[derive(Clone, Debug, PartialEq)]
    pub struct Point {
        pub x: f64,
        pub y: f64,
    }

    impl Point {
        pub fn new(x: f64, y: f64) -> Self {
            Point { x, y }
        }
    }

    impl Hash for Point {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.x.to_bits().hash(state);
            self.y.to_bits().hash(state);
        }
    }

    impl Eq for Point {}
}

fn main() {
    let mut points: Vec<models::Point> = vec![];
    let mut triangles: Vec<[usize; 3]> = vec![];
    let mut mouse_pos = [0.0, 0.0];

    let mut window: PistonWindow = WindowSettings::new("Delaunay Triangulation", [800, 600])
        .graphics_api(OpenGL::V3_2)
        .exit_on_esc(true)
        .vsync(true)
        .build()
        .unwrap();

    window.set_lazy(true);

    while let Some(e) = window.next() {
        if let Some(pos) = e.mouse_cursor_args() {
            mouse_pos = pos;
        };

        // Adiciona pontos com clique esquerdo
        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            points.push(models::Point::new(mouse_pos[0], mouse_pos[1]));
        };

        // Gera triangulação com Enter
        if let Some(Button::Keyboard(Key::Return)) = e.press_args() {
            if points.len() >= 3 {
                let del_points: Vec<DelPoint> = points.iter()
                    .map(|p| DelPoint { x: p.x, y: p.y })
                    .collect();
                
                let result = triangulate(&del_points);
                triangles = result.triangles.chunks(3)
                    .filter_map(|chunk| {
                        if chunk.len() == 3 {
                            Some([chunk[0], chunk[1], chunk[2]])
                        } else {
                            None
                        }
                    })
                    .collect();
            }
        }

        // Limpa com Backspace
        if let Some(Button::Keyboard(Key::Backspace)) = e.press_args() {
            points.clear();
            triangles.clear();
        }

        // Renderização
        window.draw_2d(&e, |context, g, _| {
            clear([1.0; 4], g);
            
            // Desenha pontos
            let red = [1.0, 0.0, 0.0, 1.0];
            for point in &points {
                Rectangle::new(red).draw(
                    [point.x - 5.0, point.y - 5.0, 10.0, 10.0],
                    &DrawState::default(),
                    context.transform,
                    g
                );
            }

            // Desenha triângulos
            let blue = [0.0, 0.0, 1.0, 1.0];
            let line = Line::new(blue, 1.0);
            for triangle in &triangles {
                if let ([a, b, c]) = triangle {
                    if let (Some(p1), Some(p2), Some(p3)) = (
                        points.get(*a),
                        points.get(*b),
                        points.get(*c),
                    ) {
                        line.draw(
                            [p1.x, p1.y, p2.x, p2.y],
                            &DrawState::default(),
                            context.transform,
                            g
                        );
                        line.draw(
                            [p2.x, p2.y, p3.x, p3.y],
                            &DrawState::default(),
                            context.transform,
                            g
                        );
                        line.draw(
                            [p3.x, p3.y, p1.x, p1.y],
                            &DrawState::default(),
                            context.transform,
                            g
                        );
                    }
                }
            }
        });
    }
}