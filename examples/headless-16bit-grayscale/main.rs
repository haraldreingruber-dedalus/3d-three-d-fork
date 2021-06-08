use three_d::Viewport;
use three_d::*;

fn main() {
    let viewport = Viewport::new_at_origo(1280, 720);

    // create headless graphic context
    let mut headless_context = HeadlessContext::new().unwrap();
    // Get the graphics context from the HeadlessContext
    let context = headless_context.gl().unwrap();

    // Create a camera
    let mut camera = Camera::new_perspective(
        &context,
        vec3(0.0, 0.0, 2.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        viewport.aspect(),
        0.1,
        10.0,
    )
    .unwrap();

    // Create a CPU-side mesh consisting of a single colored triangle
    let positions: Vec<f32> = vec![
        0.5, -0.5, 0.0, // bottom right
        -0.5, -0.5, 0.0, // bottom left
        0.0, 0.5, 0.0, // top
    ];
    let colors: Vec<u8> = vec![
        255, 0, 0, 255, // bottom right
        0, 255, 0, 255, // bottom left
        0, 0, 255, 255, // top
    ];
    let cpu_mesh = CPUMesh {
        positions,
        colors: Some(colors),
        ..Default::default()
    };

    let headless_target =
        HeadlessTarget::new(&context, Format::R16, viewport.width, viewport.height).unwrap();

    // Construct a mesh, thereby transferring the mesh data to the GPU
    let mut mesh = Mesh::new(&context, &cpu_mesh).unwrap();

    // Start the main render loop
    for frame_index in 0..3 {
        // Ensure the aspect ratio of the camera matches the aspect ratio of the window viewport
        camera.set_aspect(viewport.aspect()).unwrap();

        // Start writing to the screen and clears the color and depth
        Screen::write_with_framebuffer(
            &context,
            &headless_target.framebuffer_id,
            ClearState::color_and_depth(0.0, 0.0, 0.0, 1.0, 1.0),
            || {
                // Set the current transformation of the triangle
                mesh.transformation =
                    Mat4::from_angle_y(radians((frame_index as f32 * 0.6) as f32));

                // Render the triangle with the per vertex colors defined at construction
                mesh.render_color(RenderStates::default(), viewport, &camera)?;
                Ok(())
            },
        )
        .unwrap();

        let path = format!("headless-grayscale-{}.png", frame_index);

        let buffer_size = viewport.width * viewport.height;
        let mut pixels_r16: Vec<u16> = Vec::with_capacity(buffer_size);
        pixels_r16.resize(buffer_size, 0);

        Screen::read_r16_with_framebuffer(
            &context,
            &headless_target.framebuffer_id,
            viewport,
            &mut pixels_r16,
        )
        .unwrap();

        Saver::save_pixels_16bit_grayscale(
            path,
            pixels_r16.as_slice(),
            viewport.width,
            viewport.height,
        )
        .unwrap();
    }
}
