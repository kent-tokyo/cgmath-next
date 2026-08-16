// A minimal, realistic use of cgmath-next: build a view-projection matrix
// for a camera, then push a world-space point through it into clip space
// and normalized device coordinates -- the same per-vertex pipeline a
// software renderer or a `wgpu`/`gl` uniform upload would run.
//
// Run with: cargo run --example camera_transform

extern crate cgmath;

use cgmath::{perspective, Deg, Matrix4, Point3, Vector3, Vector4};

fn main() {
    // Camera sitting back on +Z, looking at the origin, Y up.
    let eye = Point3::new(0.0f32, 2.0, 5.0);
    let target = Point3::new(0.0, 0.0, 0.0);
    let up = Vector3::new(0.0, 1.0, 0.0);
    let view = Matrix4::look_at_rh(eye, target, up);

    // 60 degree vertical field of view, 16:9 aspect ratio.
    let aspect = 16.0 / 9.0;
    let proj = perspective(Deg(60.0f32), aspect, 0.1, 100.0);

    let view_projection = proj * view;

    // Where does a vertex at the world origin land after the camera sees it?
    let world_point = Point3::new(0.0f32, 0.0, 0.0);
    let clip = view_projection * Vector4::new(world_point.x, world_point.y, world_point.z, 1.0);

    println!("world point:  {:?}", world_point);
    println!("clip space:   {:?}", clip);

    // Perspective divide: clip space -> normalized device coordinates.
    let ndc = Vector3::new(clip.x / clip.w, clip.y / clip.w, clip.z / clip.w);
    println!("NDC:          {:?}", ndc);
}
