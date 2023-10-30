use minifb::{Key, Window, WindowOptions}; // window creation
use triangle::{Point, Triangle}; // triangle intersection math
use anyhow::Result; // error handling

const WIDTH: usize = 600;
const HEIGHT: usize = 360;

fn main() -> Result<()> {
	// create a buffer to draw a triangle in
	// u32 means that there are four numbers inside storing a value 0-255
	// * alpha (transparency)
	// * red
	// * green
	// * blue
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

	// create a window
    let mut window = Window::new(
        "Triangle!",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )?;

	// Limit the framerate to 60.24 (16600us per frame)
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
	
	//Choose your values for the coordinates.
	//x, y, z
	// x --->
	// y
	// |
	// v
	let a = Point::new(400.0,100.0,0.0);
	let b = Point::new(500.0,300.0,0.0);
	let c = Point::new(300.0,300.0,0.0);

	let triangle = Triangle::new(a,b,c);

	// run the program until the user presses escape or close the window
    while window.is_open() && !window.is_key_down(Key::Escape) {
		// run the following code on each pixel in the buffer we made
        for (index, pixel) in buffer.iter_mut().enumerate() {
			// the current x coordinate is the index modulo the width of the screen
			let x = index % WIDTH;
			// the current y coordinate is the index divided by the width of the screen
			let y = index / WIDTH;
			
			// check if our test point is within the triangle
			let test_point = Point::new(x as f32,y as f32,0.0);
		
			// color code picker: https://htmlcolorcodes.com/
			//Make sure the code starts with 0x00, which corresponds to zero alpha
			match triangle.has_point(test_point) {
				true  => *pixel = 0x00FF5733, // (Triangle color)
				false => *pixel = 0x00000000, // (Window color)
			}
        }

		// write the buffer we just filled to the screen
        window.update_with_buffer(&buffer, WIDTH, HEIGHT)?;
    };

	// exit without any errors
	Ok(())
}