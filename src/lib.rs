pub mod math;

///
///
///
pub fn create_uv_sphere_vertices(
    slice_count: u32,
    stack_count: u32,
) -> Option<(Vec<(f32, f32, f32)>, Vec<(f32, f32)>)> {
    use std::f32::consts::PI;
    if slice_count < 3 || stack_count < 2 {
        return None;
    }

    let mut vertices: Vec<(f32, f32, f32)> = vec![];
    let mut thetaphis: Vec<(f32, f32)> = vec![];

    // Add top vertex as [0].
    vertices.push((0f32, 1f32, 0f32));
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
            vertices.push((x, y, z));
            thetaphis.push((lng_rad, lat_rad));
        }
        thetaphis.push((2f32 * PI, lat_rad));
    }

    // Add bottom bertex as [E+1].
    vertices.push((0f32, -1f32, 0f32));
    thetaphis.push((0f32, 1f32));
    Some((vertices, thetaphis))
}

fn create_uv_sphere_faces_from(
    slice_count: u32,
    stack_count: u32,
    vertices: &Vec<(f32, f32, f32)>,
) -> Vec<u32> {
    let mut indices: Vec<u32> = vec![];

    // Make triangles neighboring to top[0].
    for i1 in 0..slice_count {
        let i2 = (i1 + 1) % slice_count;

        indices.push(0);
        indices.push(i1 + 1);
        indices.push(i2 + 1);
    }

    // Make quad between tringles.
    for start_idx in (0..(stack_count - 2)).map(|x| x * slice_count) {
        for offset in 0..slice_count {
            let i1 = start_idx + offset;
            let i2 = start_idx + ((offset + 1) % slice_count);
            let i3 = i1 + slice_count;
            let i4 = i2 + slice_count;

            // First triangle.
            indices.push(i2 + 1);
            indices.push(i1 + 1);
            indices.push(i3 + 1);

            // Second triangle.
            indices.push(i2 + 1);
            indices.push(i3 + 1);
            indices.push(i4 + 1);
        }
    }

    // Make triangles neighboring to bottom[E+1].
    let start = vertices.len() as u32 - slice_count;
    for i1 in 0..slice_count {
        let i2 = (i1 + 1) % slice_count;

        indices.push(start + i1 - 1);
        indices.push(vertices.len() as u32 - 1);
        indices.push(start + i2 - 1);
    }

    indices
}

///
///
///
pub fn create_uv_sphere(
    slice_count: u32,
    stack_count: u32,
) -> Option<(Vec<(f32, f32, f32)>, Vec<(f32, f32, f32)>, Vec<u32>)> {
    if slice_count < 3 || stack_count < 2 {
        return None;
    }

    let (vertices, _thetaphis) = create_uv_sphere_vertices(slice_count, stack_count).unwrap();
    let normals = vertices.clone();
    let indices = create_uv_sphere_faces_from(slice_count, stack_count, &vertices);

    Some((vertices, normals, indices))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn uv_sphere_output() {
        use std::fs::File;
        use std::io::Write;
        use std::path::Path;

        let (vertices, normals, indices) = create_uv_sphere(16, 16).unwrap();
        assert!(
            indices.len() % 3 == 0,
            "Created indices must be matched to a multiplier of 3."
        );

        for (i, face) in indices.chunks(3).enumerate() {
            println!("[{:3}] : {:?}", i, face);
        }

        // Make obj for test.
        let path = Path::new("uv_sphere_output_.obj");
        if let Ok(mut file) = File::create(&path) {
            // Vertices.
            writeln!(file, "# Vertex list").unwrap();
            for vertex in &vertices {
                let (x, y, z) = vertex;
                writeln!(file, "v {:.5} {:.5} {:.5}", x, y, z).unwrap();
            }

            // Normals.
            writeln!(file, "").unwrap();
            writeln!(file, "# Normal list").unwrap();
            for normals in &normals {
                let (x, y, z) = normals;
                writeln!(file, "vn {:.5} {:.5} {:.5}", x, y, z).unwrap();
            }

            // Indices (face)
            writeln!(file, "").unwrap();
            writeln!(file, "# Face list").unwrap();
            for f in indices.chunks(3) {
                let (v1, v2, v3) = (f[0] + 1, f[1] + 1, f[2] + 1);
                let (vn1, vn2, vn3) = (v1, v2, v3);
                writeln!(file, "f {}//{} {}//{} {}//{}", v1, vn1, v2, vn2, v3, vn3).unwrap();
            }
        }
    }
}
