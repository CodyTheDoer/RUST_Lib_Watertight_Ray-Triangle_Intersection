fn example_triangle_ray_test() {
    // Example usage
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let direction = Vec3::new(0.0, 0.0, 1.0);
    let triangle = (
        Vec3::new(1.0, 0.0, 5.0),
        Vec3::new(-1.0, 1.0, 5.0),
        Vec3::new(-1.0, -1.0, 5.0),
    );
    let backface_culling = true;

    if let Some(hit) = watertight_ray_triangle_intersection(origin, direction, triangle, backface_culling) {
        println!(
            "Intersection at t = {}, u = {}, v = {}, w = {}",
            hit.t, hit.u, hit.v, hit.w
        );
    } else {
        println!("No intersection");
    }

    // Test with reversed triangle winding
    let reversed_triangle = (
        Vec3::new(-1.0, -1.0, 5.0),
        Vec3::new(-1.0, 1.0, 5.0),
        Vec3::new(1.0, 0.0, 5.0),
    );

    if let Some(hit) = watertight_ray_triangle_intersection(origin, direction, reversed_triangle, backface_culling) {
        println!(
            "Intersection with reversed triangle at t = {}, u = {}, v = {}, w = {}",
            hit.t, hit.u, hit.v, hit.w
        );
    } else {
        println!("No intersection with reversed triangle");
    }
}
