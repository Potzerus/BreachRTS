extern crate piston_window;
extern crate image as im;
extern crate vecmath;

use piston_window::*;
use vecmath::vec2_sub;
//use vecmath::*;

const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
const COLOR_IRON: [f32; 4] = [0.1, 0.1, 0.1, 1.0];
const COLOR_FLESH: [f32; 4] = [0.7, 0.0, 0.0, 1.0];
const COLOR_WOOD: [f32; 4] = [0.3, 0.0, 0.0, 1.0];
const COLOR_GROUND: [f32; 4] = [0.5, 0.5, 0.5, 1.5];


enum Tiles {
    Iron,
    Flesh,
    Wood,
    Ground,
}

impl Tiles {
    fn get_color(&self) -> [f32; 4] {
        match self {
            Tiles::Iron => COLOR_IRON,
            Tiles::Flesh => COLOR_FLESH,
            Tiles::Wood => COLOR_WOOD,
            Tiles::Ground => COLOR_GROUND,
        }
    }
}


fn main() {
    let opengl = OpenGL::V3_2;
    let (width, height) = (400, 400);
    let grid_size = 50;
    let mut window: PistonWindow =
        WindowSettings::new("Through the Breach", (width, height))
            .graphics_api(opengl)
            .build()
            .unwrap();

    let mut is_pressed = false;

    let mut cursor: Option<[f64; 2]> = None;
    let mut first_corner: Option<[f64; 2]> = None;
    let mut live_corner: Option<[f64; 2]> = None;
    let mut _distance: Option<[f64; 2]> = None;
    while let Some(e) = window.next() {
        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            is_pressed = true;
            live_corner = cursor;
            first_corner = cursor;
        }
        if let Some(button) = e.release_args() {
            if button == Button::Mouse(MouseButton::Left) {
                is_pressed = false;
                live_corner = None
            }
        };
        if let Some(pos) = e.mouse_cursor_args() {
            cursor = Some(pos);

            if is_pressed {
                live_corner = Some(pos);
            }
        }

        if let Some(_) = e.render_args() {
            window.draw_2d(&e, |c, g, _device| {
                clear([1.0; 4], g);
                if is_pressed {
                    selection_from_points(c, g, first_corner, live_corner);
                }
            });
        }
    }
}

fn selection_from_points(c: Context, g: &mut G2d, first_corner: [f64; 2], live_corner: [f64; 2]) {
    if let Some(fc) = first_corner {
        if let Some(lc) = live_corner {
            line(GREEN,
                 1.0,
                 [fc[0], fc[1], fc[0], lc[1]],
                 c.transform, g);
            line(GREEN,
                 1.0,
                 [fc[0], fc[1], lc[0], fc[1]],
                 c.transform, g);
            line(GREEN,
                 1.0,
                 [fc[0], lc[1], lc[0], lc[1]],
                 c.transform, g);
            line(GREEN,
                 1.0,
                 [lc[0], fc[1], lc[0], lc[1]],
                 c.transform, g);
        }
    }
}


