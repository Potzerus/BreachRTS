extern crate piston_window;
extern crate image as im;
extern crate vecmath;

use piston_window::*;
use vecmath::*;

fn main() {
    let opengl = OpenGL::V3_2;
    let (width, height) = (400, 400);
    let mut window: PistonWindow =
        WindowSettings::new("Through the Breach", (width, height))
            .graphics_api(opengl)
            .build()
            .unwrap();

    let canvas = im::ImageBuffer::new(width, height);
    let mut draw = false;
    let mut texture_context = TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into(),
    };
    let mut texture: G2dTexture = Texture::from_image(
        &mut texture_context,
        &canvas,
        &TextureSettings::new(),
    ).unwrap();

    let mut first_corner: Option<[f64; 2]> = None;
    let mut distance: Option<[f64; 2]> = None;
    while let Some(e) = window.next() {
        if let Some(_) = e.render_args() {
            texture.update(&mut texture_context, &canvas).unwrap();
            window.draw_2d(&e, |c, g, device| {
                // Update texture before rendering.
                //texture_context.encoder.flush(device);

                clear([1.0; 4], g);
                if draw {
                    if let Some(fc) = first_corner {
                        if let Some(dist) = distance {
                            rectangle([0.0, 1.0, 0.0, 1.0],
                                      [fc[0], fc[1], fc[0] - dist[0], fc[1] - dist[1]],
                                      c.transform, g);
                        }
                    }
                };
                //image(&texture, c.transform, g);
            });
        }
        if let Some(button) = e.press_args() {
            if button == Button::Mouse(MouseButton::Left) {
                if draw == false {
                    draw = true;
                    first_corner = e.mouse_cursor_args()
                }
            }
        };
        if let Some(button) = e.release_args() {
            if button == Button::Mouse(MouseButton::Left) {
                draw = false;
                first_corner = None
            }
        };
        if draw {
            if let Some(pos) = e.mouse_cursor_args() {
                //let (x, y) = (pos[0] as f32, pos[1] as f32);

                if let Some(p) = first_corner {
                    //let (last_x, last_y) = (p[0] as u32, p[1] as u32);
                    distance = Some(vec2_sub(p, pos));
                    //let (distance_x, distance_y) = (distance[0] as u32, distance[1] as u32);

                    /*
                    for i in 0..distance {
                    let diff_x = x - last_x;
                    let diff_y = y - last_y;
                    let delta = i as f32 / distance as f32;
                    let new_x = (last_x + (diff_x * delta)) as u32;
                    let new_y = (last_y + (diff_y * delta)) as u32;
                    if new_x < width && new_y < height {
                        canvas.put_pixel(new_x, new_y, im::Rgba([0, 0, 0, 255]));
                        };
                    };
                    */
                };

                first_corner = Some(pos)
            };
        }
    }
}
