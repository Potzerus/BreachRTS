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

    let mut draw = false;
    let mut first_corner: Option<[f64; 2]> = None;
    let mut current: Option<[f64; 2]> = None;
    while let Some(e) = window.next() {
        if let Some(_) = e.render_args() {
            window.draw_2d(&e, |c, g, device| {
                clear([1.0; 4], g);
                if draw {
                    if let Some(curr) = current {
                        println!("{},{}", curr[0], curr[1]);
                        rectangle([0.0, 1.0, 0.0, 1.0],
                                  [curr[0], curr[1], curr[0] + 200.0, curr[1] + 200.0],
                                  c.transform, g);
                    }
                };
            });
        }
//        if let Some(button) = e.press_args() {
//            if button == Button::Mouse(MouseButton::Left) {
//                match e.mouse_cursor_args() {
//                    Some(mca) => {
//                        println!("{},{},{}", mca[0], mca[1], draw);
//                        if !draw {
//                            draw = true;
//                            let mut holder = [0.0, 0.0];
//                            holder[0] = mca[0];
//                            holder[1] = mca[1];
//                            first_corner = Some(holder);
//                            }
//                        }
//                        None => {
//                            println!("I AM NONE")
//                        }
//                }
//            }
//        };
        //if let Some(button) = e.release_args() {
        //    if button == Button::Mouse(MouseButton::Left) {
        //        draw = false;
        //        first_corner = None
        //    }
        //};
        if let Some(pos) = e.mouse_cursor_args() {
            current = Some(pos);
        } else {
            println!("Cursor args is None")
        }
    }
}
