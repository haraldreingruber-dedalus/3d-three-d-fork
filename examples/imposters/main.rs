
use window::{event::*, Window};
use dust::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let screenshot_path = if args.len() > 1 { Some(args[1].clone()) } else {None};

    let mut window = Window::new_default("Imposters!").unwrap();
    let (width, height) = window.framebuffer_size();
    let gl = window.gl();

    // Renderer
    let mut renderer = DeferredPipeline::new(&gl, width, height, vec4(0.8, 0.8, 0.8, 1.0)).unwrap();
    let mut camera = Camera::new_perspective(vec3(10.0, 25.0, 40.0), vec3(0.0, 7.0, 0.0), vec3(0.0, 1.0, 0.0),
                                                degrees(45.0), width as f32 / height as f32, 0.1, 1000.0);
    camera.enable_matrix_buffer(&gl);

    let mut loaded_objects: Vec<_> = Mesh::new_from_obj_source(&gl, include_str!("../assets/models/tree1.obj").to_string()).unwrap();
    for object in loaded_objects.iter() {
        println!("{}", object.name());
    }
    loaded_objects.pop();
    let leaves_mesh = loaded_objects.pop().unwrap();
    let tree_mesh = loaded_objects.pop().unwrap();
    let aabb = tree_mesh.axis_aligned_bounding_box().add(leaves_mesh.axis_aligned_bounding_box());
    let mut imposter = Imposter::new(&gl, &|camera: &Camera| {
            tree_mesh.render(&Mat4::identity(), camera);
            leaves_mesh.render(&Mat4::identity(), camera);
        }, (aabb.min, aabb.max));
    let t = 10;
    let mut positions = Vec::new();
    let mut angles = Vec::new();
    for x in -t..t {
        for y in -t..t {
            if x != 0 || y != 0 {
                positions.push(10.0 * x as f32);
                positions.push(0.0);
                positions.push(10.0 * y as f32);
                angles.push((1.0 + y as f32 / t as f32) * std::f32::consts::PI);
            }
        }
    }
    imposter.update_positions(&positions, &angles);

    let mut plane = Mesh::new_plane(&gl).unwrap();
    plane.diffuse_intensity = 0.5;
    plane.specular_intensity = 0.8;

    renderer.ambient_light().set_intensity(0.1);

    let mut directional_light = renderer.directional_light(0).unwrap();
    directional_light.set_direction(&vec3(1.0, -1.0, -1.0));
    directional_light.set_color(&vec3(1.0, 0.0, 0.0));
    directional_light.set_intensity(0.5);
    directional_light.enable_shadows();
    directional_light.update_shadows(vec3(0.0, 0.0, 0.0), 300.0, 300.0);

    directional_light = renderer.directional_light(1).unwrap();
    directional_light.set_direction(&vec3(-1.0, -1.0, 1.0));
    directional_light.set_color(&vec3(0.0, 1.0, 0.0));
    directional_light.set_intensity(0.5);
    directional_light.enable_shadows();
    directional_light.update_shadows(vec3(0.0, 0.0, 0.0), 300.0, 300.0);

    let mut debug_effect = effects::DebugEffect::new(&gl).unwrap();

    // main loop
    let mut rotating = false;
    window.render_loop(move |frame_input|
    {
        camera.set_size(frame_input.screen_width as f32, frame_input.screen_height as f32);

        for event in frame_input.events.iter() {
            match event {
                Event::MouseClick {state, button, ..} => {
                    rotating = *button == MouseButton::Left && *state == State::Pressed;
                },
                Event::MouseMotion {delta} => {
                    if rotating {
                        camera.rotate(delta.0 as f32, delta.1 as f32);
                    }
                },
                Event::MouseWheel {delta} => {
                    camera.zoom(*delta as f32);
                },
                Event::Key { ref state, ref kind } => {
                    if kind == "R" && *state == State::Pressed
                    {
                        debug_effect.change_type();
                    }
                }
            }
        }

        // Draw
        let render_scene = |camera: &Camera| {
            tree_mesh.render(&Mat4::identity(), camera);
            leaves_mesh.render(&Mat4::identity(), camera);
            imposter.render(&camera);
        };

        // Shadow pass
        renderer.shadow_pass(&|_|{});

        // Geometry pass
        renderer.geometry_pass(&||
            {
                render_scene(&camera);
                plane.render(&(Mat4::from_scale(100.0)), &camera);
            }).unwrap();

        // Light pass
        renderer.light_pass(&camera).unwrap();
        debug_effect.apply(&camera, renderer.geometry_pass_texture(), renderer.geometry_pass_depth_texture()).unwrap();

        if let Some(ref path) = screenshot_path {
            #[cfg(target_arch = "x86_64")]
            save_screenshot(path, &gl, width, height).unwrap();
            std::process::exit(1);
        }

    }).unwrap();
}