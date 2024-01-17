extern crate kiss3d;

use kiss3d::nalgebra::{Vector3, UnitQuaternion, Translation3};
use kiss3d::window::Window;
use kiss3d::light::Light;

use std::time::Duration;
use std::thread::sleep;

fn main() {
    let mut window = Window::new("Cubes");
    let mut c1 = window.add_cube(1.0, 1.0, 1.0);
    let mut c2 = c1.add_cube(0.75, 0.75, 0.75);
    let mut c3 = c2.add_cube(0.5, 0.5, 0.5);
    let mut c4 = c3.add_cube(0.25, 0.25, 0.25);

    c1.set_color(1.0, 0.0, 0.0);
    c1.set_color(0.25, 0.0, 0.0);
    c1.set_color(0.5, 0.0, 0.0);
    c1.set_color(0.75, 0.0, 0.0);

    window.set_light(Light::StickToCamera);

    let rot_y = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.01);
    let rot_x = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), 0.01);
    let move_x = Translation3::new(0.5, 0.0, 0.0);
    
    c2.prepend_to_local_translation(&move_x);
    c3.prepend_to_local_translation(&move_x);
    c4.prepend_to_local_translation(&move_x);

    let mut i: u64 = 0;
    let mut b = true;

    while window.render() {
        
        if i == 0 {
            println!("Swapping color. i = {i}");
            if b {
                c1.set_color(1.0, 0.0, 0.0);
            } else {
                c1.set_color(0.0, 0.0, 1.0);
            }
            b = !b;
        }
        if i % 10 == 0 { println!("{i}") }

        let r = if b {
            &rot_y
        } else {
            &rot_x
        };
        
        sleep(Duration::from_secs_f32(0.1));

        c1.prepend_to_local_rotation(r);
        c2.prepend_to_local_rotation(r);
        c3.prepend_to_local_rotation(r);
        c4.prepend_to_local_rotation(r);
        
        i += 1;
        i %= 50;
        
    }
}
