use bresenham::Bresenham;
use cgmath::{Matrix3, Rad, Vector3};
use crossterm::{cursor, QueueableCommand};
use std::{io::stdout, usize};

pub type Coord2D = (isize, isize);
pub type Point2D = (f64, f64);
pub type Point3D = (f64, f64, f64);

struct Buffer {
    pub buffer: Vec<char>,
    pub rows: u8,
    pub columns: u8,
    pub ever_rendered: bool,
}
impl Buffer {
    pub fn clear(&mut self) {
        self.buffer.fill('.');
    }
    pub fn render_buffer(&mut self) {
        let mut stdout = stdout();
        if self.ever_rendered {
            stdout.queue(cursor::MoveUp(self.rows as u16 + 1)).unwrap();
        } else {
            self.ever_rendered = true;
        }
        let mut string_to_render = String::new();
        for i in 0..self.rows as usize {
            for j in 0..self.columns as usize {
                string_to_render.push(self.buffer[i * self.columns as usize + j]);
            }
            string_to_render.push('\n');
        }
        println!("{}", string_to_render);
    }
    pub fn draw_line(&mut self, start: Coord2D, end: Coord2D) {
        for (x, y) in Bresenham::new(start, end) {
            self.buffer
                [(self.rows as usize - (y as usize)) * (self.columns as usize) + (x as usize)] =
                '#';
        }
    }
    /*
    pub fn draw_polygon(&mut self, vertices: Vec<Coord2D>) {
        for i in 0..vertices.len() - 1 {
            self.draw_line(vertices[i], vertices[i + 1]);
        }
        self.draw_line(*vertices.last().unwrap(), vertices[0]);
    }*/

    pub fn point2d_to_coord2d(&self, (x, y): Point2D) -> Coord2D {
        let rowsf64 = self.rows as f64;
        let columnsf64 = self.columns as f64;
        (
            (x * columnsf64 / 2.0 + columnsf64 / 2.0).round() as isize,
            (y * rowsf64 / 2.0 + rowsf64 / 2.0).round() as isize,
        )
    }
    pub fn draw_skeleton(&mut self, skeleton: &Skeleton) {
        let vertices = skeleton
            .vertices
            .iter()
            .map(|(x, y, z)| self.point2d_to_coord2d((x / (z + 2.0), y / (z + 2.0))))
            .collect::<Vec<Coord2D>>();
        for (start, end) in &skeleton.edges {
            self.draw_line(vertices[*start], vertices[*end])
        }
    }
}

pub struct Skeleton {
    pub vertices: Vec<Point3D>,
    pub edges: Vec<(usize, usize)>,
}

impl Skeleton {
    pub fn rotate_x(&mut self, angle: f64) {
        let rotation_matrix = Matrix3::from_angle_x(Rad(angle));
        for point in self.vertices.iter_mut() {
            let old_point: Vector3<f64> = (*point).into();
            let new_point: Vector3<f64> = rotation_matrix * old_point;
            *point = (new_point.x, new_point.y, new_point.z);
        }
    }

    pub fn rotate_y(&mut self, angle: f64) {
        let rotation_matrix = Matrix3::from_angle_y(Rad(angle));
        for point in self.vertices.iter_mut() {
            let old_point: Vector3<f64> = (*point).into();
            let new_point: Vector3<f64> = rotation_matrix * old_point;
            *point = (new_point.x, new_point.y, new_point.z);
        }
    }
    pub fn rotate_z(&mut self, angle: f64) {
        let rotation_matrix = Matrix3::from_angle_z(Rad(angle));
        for point in self.vertices.iter_mut() {
            let old_point: Vector3<f64> = (*point).into();
            let new_point: Vector3<f64> = rotation_matrix * old_point;
            *point = (new_point.x, new_point.y, new_point.z);
        }
    }
}
fn main() {
    let columns: u8 = 200;
    let rows: u8 = 50;
    let mut buffer = Buffer {
        buffer: vec!['.'; columns as usize * rows as usize],
        columns,
        rows,
        ever_rendered: false,
    };

    /*
    Vertices are ordered like this:
      p6 ----- p7
      /|       /
     / |      /|
    p1 ----- p2|
    |  p5----|-p8
    | /      | /
    |/       |/
    p4 ----- p3


    let mut cube_skeleton = Skeleton {
        vertices: vec![
            (-0.5, 0.5, -0.5),
            (0.5, 0.5, -0.5),
            (0.5, -0.5, -0.5),
            (-0.5, -0.5, -0.5),
            (-0.5, -0.5, 0.5),
            (-0.5, 0.5, 0.5),
            (0.5, 0.5, 0.5),
            (0.5, -0.5, 0.5),
        ],
        edges: vec![
            (4, 5),
            (5, 6),
            (6, 7),
            (4, 7),
            (2, 7),
            (3, 4),
            (0, 5),
            (1, 6),
            (0, 1),
            (1, 2),
            (2, 3),
            (0, 3),
        ],
    };
    */
    let mut pyramid_skeleton = Skeleton {
        vertices: vec![
            (0.0, 1.0, 0.0),
            (-0.5, -0.5, -0.5),
            (0.5, -0.5, -0.5),
            (0.5, -0.5, 0.5),
            (-0.5, -0.5, 0.5),
        ],
        edges: vec![
            (0, 1),
            (0, 2),
            (0, 3),
            (0, 4),
            (1, 2),
            (2, 3),
            (3, 4),
            (4, 1),
        ],
    };
    loop {
        pyramid_skeleton.rotate_x(0.005);
        buffer.clear();
        buffer.draw_skeleton(&pyramid_skeleton);
        buffer.render_buffer();
    }
}
