pub mod math;
use math::Vec3;
use std::fmt::Debug;

///
///
///
#[derive(Debug)]
pub struct Vertex {
    pub position_index: u32,
    pub uv0_index: Option<u32>,
    pub normal_index: Option<u32>,
}

impl Vertex {
    ///
    pub(crate) fn from_position(index: u32) -> Self {
        Self {
            position_index: index,
            uv0_index: None,
            normal_index: None,
        }
    }

    ///
    pub(crate) fn with_normal(self, index: u32) -> Self {
        Self {
            position_index: self.position_index,
            uv0_index: self.uv0_index,
            normal_index: Some(index),
        }
    }
}

///
///
///
#[derive(Debug)]
pub struct Face {
    datas: [Vertex; 3],
}

impl Face {
    /// Create single face being consisted by three vertices which have indices to actual information.
    pub(crate) fn from_triangle(first: Vertex, second: Vertex, third: Vertex) -> Self {
        Self {
            datas: [first, second, third],
        }
    }

    ///
    ///
    ///
    pub fn visit_enumerated<F>(&self, mut f: F)
    where
        F: for<'any> FnMut(usize, &'any Vertex) -> (),
    {
        for (i, data) in self.datas.iter().enumerate() {
            f(i, data);
        }
    }
}

///
///
///
pub struct UVSphere {
    ///
    positions: Vec<Vec3>,
    ///
    uv0s: Option<Vec<Vec3>>,
    ///
    normals: Option<Vec<Vec3>>,
    ///
    faces: Option<Vec<Face>>,
}

impl UVSphere {
    ///
    ///
    ///
    pub fn create_from(slice_count: u32, stack_count: u32) -> Option<Self> {
        if slice_count < 3 || stack_count < 2 {
            return None;
        }

        let (vertices, _thetaphis) = create_uv_sphere_vertices(slice_count, stack_count).unwrap();
        let normals = vertices.clone();
        let faces = create_uv_sphere_faces_from(slice_count, stack_count, &vertices);

        Some(Self {
            positions: vertices,
            uv0s: None,
            normals: Some(normals),
            faces: Some(faces),
        })
    }

    ///
    ///
    ///
    pub fn positions(&self) -> &Vec<Vec3> {
        &self.positions
    }

    ///
    ///
    ///
    pub fn uv0s(&self) -> Option<&Vec<Vec3>> {
        self.uv0s.as_ref()
    }

    ///
    ///
    ///
    pub fn normals(&self) -> Option<&Vec<Vec3>> {
        self.normals.as_ref()
    }

    ///
    ///
    ///
    pub fn faces(&self) -> Option<&Vec<Face>> {
        self.faces.as_ref()
    }
}

///
///
///
fn create_uv_sphere_vertices(
    slice_count: u32,
    stack_count: u32,
) -> Option<(Vec<Vec3>, Vec<(f32, f32)>)> {
    use std::f32::consts::PI;
    if slice_count < 3 || stack_count < 2 {
        return None;
    }

    let mut vertices: Vec<Vec3> = vec![];
    let mut thetaphis: Vec<(f32, f32)> = vec![];

    // Add top vertex as [0].
    vertices.push(Vec3::unit_y());
    thetaphis.push((0f32, 0f32));

    // Add vertices using slice (longitude (vertical)) and stack (latitude (parallel)).
    // [1.. E := slice*(stack-1)].
    for lati in 1..stack_count {
        let lat_rad = (PI * lati as f32) / (stack_count as f32);
        for long in 0..slice_count {
            let lng_rad = (2f32 * PI * long as f32) / (slice_count as f32);

            let x = lat_rad.sin() * lng_rad.sin();
            let y = lat_rad.cos();
            let z = lat_rad.sin() * lng_rad.cos();
            vertices.push(Vec3::new(x, y, z));
            thetaphis.push((lng_rad, lat_rad));
        }
        thetaphis.push((2f32 * PI, lat_rad));
    }

    // Add bottom bertex as [E+1].
    vertices.push(Vec3::unit_y() * -1f32);
    thetaphis.push((0f32, 1f32));
    Some((vertices, thetaphis))
}

fn create_uv_sphere_faces_from(
    slice_count: u32,
    stack_count: u32,
    vertices: &Vec<Vec3>,
) -> Vec<Face> {
    let mut faces: Vec<Face> = vec![];

    // Make triangles neighboring to top[0].
    for i1 in 0..slice_count {
        let i2 = (i1 + 1) % slice_count;

        faces.push(Face::from_triangle(
            Vertex::from_position(0).with_normal(0),
            Vertex::from_position(i1 + 1).with_normal(i1 + 1),
            Vertex::from_position(i2 + 1).with_normal(i2 + 1),
        ));
    }

    // Make quad between tringles.
    for start_idx in (0..(stack_count - 2)).map(|x| x * slice_count) {
        for offset in 0..slice_count {
            let i1 = start_idx + offset;
            let i2 = start_idx + ((offset + 1) % slice_count);
            let i3 = i1 + slice_count;
            let i4 = i2 + slice_count;

            // First triangle.
            faces.push(Face::from_triangle(
                Vertex::from_position(i2 + 1).with_normal(i2 + 1),
                Vertex::from_position(i1 + 1).with_normal(i1 + 1),
                Vertex::from_position(i3 + 1).with_normal(i3 + 1),
            ));

            // Second triangle.
            faces.push(Face::from_triangle(
                Vertex::from_position(i2 + 1).with_normal(i2 + 1),
                Vertex::from_position(i3 + 1).with_normal(i3 + 1),
                Vertex::from_position(i4 + 1).with_normal(i4 + 1),
            ));
        }
    }

    // Make triangles neighboring to bottom[E+1].
    let start = vertices.len() as u32 - slice_count;
    for i1 in 0..slice_count {
        let i2 = (i1 + 1) % slice_count;

        faces.push(Face::from_triangle(
            Vertex::from_position(start + i1 - 1).with_normal(start + i1 - 1),
            Vertex::from_position(vertices.len() as u32 - 1).with_normal(vertices.len() as u32 - 1),
            Vertex::from_position(start + i2 - 1).with_normal(start + i2 - 1),
        ));
    }

    faces
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn uv_sphere_output() {
        use std::fs::File;
        use std::io::Write;
        use std::path::Path;

        let uv_sphere = UVSphere::create_from(16, 16).unwrap();
        assert!(
            uv_sphere.faces().unwrap().len() % 3 == 0,
            "Created indices must be matched to a multiplier of 3."
        );

        for (i, face) in uv_sphere.faces().unwrap().chunks(3).enumerate() {
            println!("[{:3}] : {:?}", i, face);
        }

        // Make obj for test.
        let path = Path::new("uv_sphere_output_.obj");
        if let Ok(mut file) = File::create(&path) {
            // Vertices.
            writeln!(file, "# Vertex list").unwrap();
            for vertex in uv_sphere.positions() {
                let (x, y, z) = (vertex[0], vertex[1], vertex[2]);
                writeln!(file, "v {:.5} {:.5} {:.5}", x, y, z).unwrap();
            }

            // Normals.
            writeln!(file, "").unwrap();
            writeln!(file, "# Normal list").unwrap();
            for normals in uv_sphere.normals().unwrap() {
                let (x, y, z) = (normals[0], normals[1], normals[2]);
                writeln!(file, "vn {:.5} {:.5} {:.5}", x, y, z).unwrap();
            }

            // Indices (face)
            writeln!(file, "").unwrap();
            writeln!(file, "# Face list").unwrap();
            for f in uv_sphere.faces().unwrap() {
                write!(file, "f ").unwrap();
                f.visit_enumerated(|_, vertex: &Vertex| {
                    write!(
                        file,
                        "{}//{} ",
                        vertex.position_index + 1,
                        vertex.normal_index.unwrap() + 1
                    )
                    .unwrap();
                    //let (v1, v2, v3) = (f[0] + 1, f[1] + 1, f[2] + 1);
                    //let (vn1, vn2, vn3) = (v1, v2, v3);
                    //writeln!(file, "f {}//{} {}//{} {}//{}", v1, vn1, v2, vn2, v3, vn3).unwrap();
                });
                writeln!(file, "").unwrap();
            }
        }
    }
}
