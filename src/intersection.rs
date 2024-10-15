use glam::Vec3;

pub struct Hit {
    u: f32,
    v: f32,
    w: f32,
    t: f32,
}

impl Hit {
    pub fn as_tuple(&self) -> (f32, f32, f32, f32) {
        (self.u, self.v, self.w, self.t)
    }
    
    // Getter methods for each field
    pub fn u(&self) -> f32 {
        self.u
    }

    pub fn v(&self) -> f32 {
        self.v
    }

    pub fn w(&self) -> f32 {
        self.w
    }

    pub fn t(&self) -> f32 {
        self.t
    }
}

pub fn watertight_ray_triangle_intersection(
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn internal_test_intersection_direct() {
        let origin = Vec3::new(0.0, 0.0, 0.0);
        let direction = Vec3::new(0.0, 0.0, 1.0);
        let triangle = (
            Vec3::new(1.0, 0.0, 5.0),
            Vec3::new(-1.0, 1.0, 5.0),
            Vec3::new(-1.0, -1.0, 5.0),
        );
        let backface_culling = false;

        let result = watertight_ray_triangle_intersection(origin, direction, triangle, backface_culling);
        assert!(result.is_some());
        let hit = result.unwrap();
        assert!(hit.t() > 0.0);
        assert!((hit.u() + hit.v() + hit.w() - 1.0).abs() < 1e-5);
    }

    #[test]
    fn internal_test_intersection_some() {
        let origin = Vec3::new(0.0, 0.0, 0.0);
        let direction = Vec3::new(0.0, 0.0, 1.0);
        let triangle = (
            Vec3::new(-1.0, -1.0, 5.0),
            Vec3::new(-1.0, 1.0, 5.0),
            Vec3::new(1.0, 0.0, 5.0),
        );
        let backface_culling = true;

        if let Some(hit) = watertight_ray_triangle_intersection(origin, direction, triangle, backface_culling) {
            let (t, u, v, w) = hit.as_tuple();

            // You can call all values directly:
            // let t = hit.t();
            // let u = hit.u();
            // let v = hit.v();
            // let w = hit.w();

            println!(
                "Intersection at t = {}, u = {}, v = {}, w = {}",
                t, u, v, w
            );
            assert!(t > 0.0);
        } else {
            panic!("Expected an intersection, but got None");
        }
    }

    #[test]
    fn internal_test_no_intersection() {
        let origin = Vec3::new(0.0, 0.0, 0.0);
        let direction = Vec3::new(0.0, 1.0, 0.0); // Ray is parallel to Z axis
        let triangle = (
            Vec3::new(1.0, 0.0, 5.0),
            Vec3::new(-1.0, 1.0, 5.0),
            Vec3::new(-1.0, -1.0, 5.0),
        );
        let backface_culling = true;

        assert!(watertight_ray_triangle_intersection(origin, direction, triangle, backface_culling).is_none());
    }
}