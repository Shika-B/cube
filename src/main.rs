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
    pub fn draw_polygon(&mut self, vertices: Vec<Coord2D>) {
        for i in 0..vertices.len() - 1 {
            self.draw_line(vertices[i], vertices[i + 1]);
        }
        self.draw_line(*vertices.last().unwrap(), vertices[0]);
    }
    pub fn point2d_to_coord2d(&self, (x, y): Point2D) -> Coord2D {
        let rowsf64 = self.rows as f64;
        let columnsf64 = self.columns as f64;
        (
            (x * columnsf64 / 2.0 + columnsf64 / 2.0).round() as isize,
            (y * rowsf64 / 2.0 + rowsf64 / 2.0).round() as isize,
        )
    }
    pub fn draw_cube(&mut self, cube: &Cube) {
        let vertices = [
            self.point2d_to_coord2d((
                cube.vertices[0].0 / (cube.vertices[0].2 + 2.0),
                cube.vertices[0].1 / (cube.vertices[0].2 + 2.0),
            )),
            self.point2d_to_coord2d((
                cube.vertices[1].0 / (cube.vertices[1].2 + 2.0),
                cube.vertices[1].1 / (cube.vertices[1].2 + 2.0),
            )),
            self.point2d_to_coord2d((
                cube.vertices[2].0 / (cube.vertices[2].2 + 2.0),
                cube.vertices[2].1 / (cube.vertices[2].2 + 2.0),
            )),
            self.point2d_to_coord2d((
                cube.vertices[3].0 / (cube.vertices[3].2 + 2.0),
                cube.vertices[3].1 / (cube.vertices[3].2 + 2.0),
            )),
            self.point2d_to_coord2d((
                cube.vertices[4].0 / (cube.vertices[4].2 + 2.0),
                cube.vertices[4].1 / (cube.vertices[4].2 + 2.0),
            )),
            self.point2d_to_coord2d((
                cube.vertices[5].0 / (cube.vertices[5].2 + 2.0),
                cube.vertices[5].1 / (cube.vertices[5].2 + 2.0),
            )),
            self.point2d_to_coord2d((
                cube.vertices[6].0 / (cube.vertices[6].2 + 2.0),
                cube.vertices[6].1 / (cube.vertices[6].2 + 2.0),
            )),
            self.point2d_to_coord2d((
                cube.vertices[7].0 / (cube.vertices[7].2 + 2.0),
                cube.vertices[7].1 / (cube.vertices[7].2 + 2.0),
            )),
        ];
        self.draw_line(vertices[4], vertices[5]);
        self.draw_line(vertices[5], vertices[6]);
        self.draw_line(vertices[6], vertices[7]);
        self.draw_line(vertices[4], vertices[7]);

        self.draw_line(vertices[2], vertices[7]);
        self.draw_line(vertices[3], vertices[4]);
        self.draw_line(vertices[0], vertices[5]);
        self.draw_line(vertices[1], vertices[6]);

        self.draw_line(vertices[0], vertices[1]);
        self.draw_line(vertices[1], vertices[2]);
        self.draw_line(vertices[2], vertices[3]);
        self.draw_line(vertices[0], vertices[3]);
    }
}

/*
  p6 ----- p7
  /|       /
 / |      /|
p1 ----- p2|
|  p5----|-p8
| /      | /
|/       |/
p4 ----- p3

*/
pub struct Cube {
    pub vertices: [Point3D; 8],
}

impl Cube {
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
    let mut cube = Cube {
        vertices: [
            (-0.5, 0.5, -0.5),
            (0.5, 0.5, -0.5),
            (0.5, -0.5, -0.5),
            (-0.5, -0.5, -0.5),
            (-0.5, -0.5, 0.5),
            (-0.5, 0.5, 0.5),
            (0.5, 0.5, 0.5),
            (0.5, -0.5, 0.5),
        ],
    };
    buffer.draw_cube(&cube);
    loop {
        cube.rotate_x(0.005);
        cube.rotate_z(0.002);
        buffer.clear();
        buffer.draw_cube(&cube);
        buffer.render_buffer();
    }
}
