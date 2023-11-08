use macroquad::prelude::*;

const RADIUS: f32 = 15.0;

fn pick_color(v: [f32; 2]) -> Color {
    Color::from_rgba(
        rand::gen_range(50, 240),
        rand::gen_range(50, 240),
        rand::gen_range(50, 240),
        255,
    )
}

fn mouse_difference(x: f32, y: f32) -> [f32; 2] {
    let mouse_pos = mouse_position();
    [x - mouse_pos.0, y - mouse_pos.1]
}

fn acceleration(x: f32, y: f32, vel: [f32; 2]) -> [f32; 2] {
    // gravity
    let r = mouse_difference(x, y);
    let mut ddx = -r[0] * 0.01;
    let mut ddy = -r[1] * 0.01;
    // soup
    ddx += -0.01 * vel[0];
    ddy += -0.01 * vel[1];
    [ddx, ddy]
}

fn velocity(x: f32, y: f32, vel: [f32; 2]) -> [f32; 2] {
    let mut vel = vel.clone();
    let a = acceleration(x, y, vel);
    vel[0] += a[0];
    vel[1] += a[1];
    vel
}

#[macroquad::main("OneDot")]
async fn main() {
    let mut frame_count = 0;
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;
    let mut v = [0.0, 0.0];

//    let mut color = color_u8!(128, 128, 128, 255);
    let mut color = pick_color(v);
    let mut old_screen_img = get_screen_data();
    let old_screen = Texture2D::from_image(&old_screen_img);

    loop {
        // Update
        v = velocity(x, y, v);
        x += v[0];
        y += v[1];

        // Draw
        draw_texture_ex(
            &old_screen,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                flip_y: true,
                ..Default::default()
            },
        );

        draw_circle(x, y, RADIUS, color);

        // Update Trails
        if frame_count == 2 {
            frame_count = 0;
            old_screen_img = get_screen_data();
            old_screen.update(&old_screen_img);
            color = pick_color(v); // less flashing
        } else {
            frame_count += 1;
        }

        next_frame().await
    }
}
