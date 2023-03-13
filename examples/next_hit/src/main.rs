fn main() {
    let display = Display::new(DISPLAY_WIDTH, DISPLAY_HEIGHT, "triangle geometry");
    let device = Device::new().unwrap();
    device.set_error_function(|err, msg| {
        println!("{}: {}", err, msg);
    });
    let scene = device.create_scene().unwrap();
    let vertex_colors = vec![
        [0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 1.0],
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 1.0],
        [1.0, 1.0, 0.0],
        [1.0, 1.0, 1.0],
    ];

    let user_state = UserState {
        face_colors: vec![
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.5, 0.5, 0.5],
            [0.5, 0.5, 0.5],
            [1.0, 1.0, 1.0],
            [1.0, 1.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [1.0, 1.0, 0.0],
            [1.0, 1.0, 0.0],
        ],
        ground_id: INVALID_ID,
        cube_id: INVALID_ID,
        light_dir: Vec3::new(1.0, 1.0, 1.0).normalize(),
    };

    let mut state = State {
        scene: scene.clone(),
        user: user_state,
    };

    let cube = make_cube(&device, &vertex_colors);
    let ground = make_ground_plane(&device);
    state.user.cube_id = state.scene.attach_geometry(&cube);
    state.user.ground_id = state.scene.attach_geometry(&ground);

    state.scene.commit();

    display::run(display, state, move |_, _| {}, render_frame, |_| {});
}
