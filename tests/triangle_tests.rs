#[cfg(test)]
mod tests {
    use wrti_library::watertight_ray_triangle_intersection;
    use glam::Vec3;
    #[test]
    fn integration_test_intersection_direct() {
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
    fn integration_test_intersection_some() {
        let origin = Vec3::new(0.0, 0.0, 0.0);
        let direction = Vec3::new(0.0, 0.0, 1.0);
        let triangle = (
            Vec3::new(-1.0, -1.0, 5.0),
            Vec3::new(-1.0, 1.0, 5.0),
            Vec3::new(1.0, 0.0, 5.0),
        );
        let backface_culling = true;

        if let Some(hit) = watertight_ray_triangle_intersection(origin, direction, triangle, backface_culling) {
            let t = hit.t();
            let u = hit.u();
            let v = hit.v();
            let w = hit.w();
            // More verbose way to call all values directly:
            // let (t, u, v, w) = hit.as_tuple();

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
    fn no_intersection() {
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