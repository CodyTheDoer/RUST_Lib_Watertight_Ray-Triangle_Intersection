use glam::Vec3;

struct Hit {
    u: f32,
    v: f32,
    w: f32,
    t: f32,
}

fn watertight_ray_triangle_intersection(
    origin: Vec3,                   // Ray origin
    direction: Vec3,                // Ray direction
    triangle: (Vec3, Vec3, Vec3),   // Triangle vertices
    backface_culling: bool,         // Backface culling flag
) -> Option<Hit> {
    // Calculate dimension where the ray direction is maximal
    fn index_max_abs_dim(v: Vec3) -> usize {
        let abs_v = v.abs();
        if abs_v.x >= abs_v.y && abs_v.x >= abs_v.z {
            0
        } else if abs_v.y >= abs_v.x && abs_v.y >= abs_v.z {
            1
        } else {
            2
        }
    }

    let kz = index_max_abs_dim(direction);
    let mut kx = (kz + 1) % 3;
    let mut ky = (kx + 1) % 3;

    // Swap kx and ky to preserve winding direction of triangles
    if direction[kz] < 0.0 {
        std::mem::swap(&mut kx, &mut ky);
    }

    let f32_epsilon_check = std::f32::EPSILON;

    // Ensure we're not dividing by zero
    if direction[kz].abs() < f32_epsilon_check {
        return None;
    }

    // Calculate shear constants
    let sx: f32 = direction[kx] / direction[kz];
    let sy: f32 = direction[ky] / direction[kz];
    let sz: f32 = 1.0 / direction[kz];

    // Calculate vertices relative to ray origin
    let point_a = triangle.0 - origin;
    let point_b = triangle.1 - origin;
    let point_c = triangle.2 - origin;

    // Perform shear and scale of vertices
    let point_a_x = point_a[kx] - sx * point_a[kz];
    let point_a_y = point_a[ky] - sy * point_a[kz];
    let point_b_x = point_b[kx] - sx * point_b[kz];
    let point_b_y = point_b[ky] - sy * point_b[kz];
    let point_c_x = point_c[kx] - sx * point_c[kz];
    let point_c_y = point_c[ky] - sy * point_c[kz];

    // Calculate scaled barycentric coordinates
    let mut u = point_c_x * point_b_y - point_c_y * point_b_x;
    let mut v = point_a_x * point_c_y - point_a_y * point_c_x;
    let mut w = point_b_x * point_a_y - point_b_y * point_a_x;

    // Fallback to test against edges using double precision
    if u.abs() < f32_epsilon_check || v.abs() < f32_epsilon_check || w.abs() < f32_epsilon_check {
        let cx_by = (point_c_x as f64) * (point_b_y as f64);
        let cy_bx = (point_c_y as f64) * (point_b_x as f64);
        u = (cx_by - cy_bx) as f32;

        let ax_cy = (point_a_x as f64) * (point_c_y as f64);
        let ay_cx = (point_a_y as f64) * (point_c_x as f64);
        v = (ax_cy - ay_cx) as f32;

        let bx_ay = (point_b_x as f64) * (point_a_y as f64);
        let by_ax = (point_b_y as f64) * (point_a_x as f64);
        w = (bx_ay - by_ax) as f32;
    }

    // Calculate normal of the triangle to determine orientation
    let edge1 = triangle.1 - triangle.0;
    let edge2 = triangle.2 - triangle.0;
    let normal = edge1.cross(edge2);
    let facing = normal.dot(direction);

    // Log triangle orientation
    println!("Triangle normal: {:?}, Ray direction dot normal: {}", normal, facing);

    // Perform edge tests
    if backface_culling {
        if u < 0.0 || v < 0.0 || w < 0.0 {
            println!("Backface culling enabled: Ray hit the back of the triangle");
            return None;
        }
    } else {
        if (u < 0.0 || v < 0.0 || w < 0.0) && (u > 0.0 || v > 0.0 || w > 0.0) {
            return None;
        }
    }

    // Calculate determinant
    let mut det = u + v + w;
    if det == 0.0 {
        return None;
    }

    // Handle negative determinant
    if det < 0.0 {
        u = -u;
        v = -v;
        w = -w;
        det = -det;
    }

    // Calculate scaled z-coordinates of vertices and use them to calculate the hit distance
    let point_a_z = sz * point_a[kz];
    let point_b_z = sz * point_b[kz];
    let point_c_z = sz * point_c[kz];
    let mut t = u * point_a_z + v * point_b_z + w * point_c_z;

    // Apply sign flipping if necessary
    fn sign_mask(f: f32) -> u32 {
        (f.to_bits() >> 31) & 1 // returns 1 if f is negative, and 0 if positive
    }
    fn xorf(value: f32, sign_mask: u32) -> f32 {
        let value_bits = value.to_bits();
        let result_bits = value_bits ^ (sign_mask << 31);
        f32::from_bits(result_bits) // returns value with flipped sign if determinant is negative
    }

    let det_sign = sign_mask(det);
    t = xorf(t, det_sign);
    if t < 0.0 {
        return None;
    }

    // Normalize U, V, W, and T
    let reciprocal_det = 1.0 / det;
    let hit = Hit {
        u: u * reciprocal_det,
        v: v * reciprocal_det,
        w: w * reciprocal_det,
        t: t * reciprocal_det,
    };

    Some(hit)
}