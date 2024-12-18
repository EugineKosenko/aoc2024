use std::{fs, env, io::{self, BufRead}};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::ttf::Font;
use std::time::Duration;

lazy_static::lazy_static! {
    static ref RE_ROBOT: regex::Regex = regex::Regex::new(r"^p=(?P<px>-?\d+),(?P<py>-?\d+) v=(?P<vx>-?\d+),(?P<vy>-?\d+)$").unwrap();
}

fn main() -> Result<(), String> {
    // Read input file
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let lines = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap());
    
    // Initialize coordinates and velocities
    let mut coordinates = vec![];
    let mut velocities = vec![];

    for line in lines {
        let cps = RE_ROBOT.captures(&line).unwrap();
        let px = cps.name("px").unwrap().as_str().parse::<i32>().unwrap();
        let py = cps.name("py").unwrap().as_str().parse::<i32>().unwrap();
        let vx = cps.name("vx").unwrap().as_str().parse::<i32>().unwrap();
        let vy = cps.name("vy").unwrap().as_str().parse::<i32>().unwrap();
        coordinates.push((px, py));
        velocities.push((vx, vy));
    }

    // Maximum coordinate values (board dimensions)
    let max_x = 101;
    let max_y = 103;

    // Window dimensions
    let window_width = 505;
    let window_height = 515;

    // Initialize SDL2
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // Initialize SDL2_ttf for text rendering
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font_path = "/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf";
    let font = ttf_context.load_font(font_path, 32)?;

    // Create SDL window and canvas
    let window = video_subsystem
        .window("Coordinate Animation", window_width, window_height)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas: Canvas<Window> = window.into_canvas().build().map_err(|e| e.to_string())?;
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.present();

    // Scaling factors
    let scale_x = (window_width as f32 / max_x as f32) as i32;
    let scale_y = (window_height as f32 / max_y as f32) as i32;

    // Animation state variables
    let mut step = 0;
    let mut running = true; // Animation running or paused
    let mut frame_delay = 100; // Milliseconds per frame
    let mut history = vec![coordinates.clone()]; // History to support stepping back

    // Start event loop
    let mut event_pump: EventPump = sdl_context.event_pump()?;
    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            use sdl2::event::Event;
            use sdl2::keyboard::Keycode;
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    // Toggle pause/resume on spacebar
                    running = !running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    // If running, speed up animation
                    if running {
                        if frame_delay > 10 {
                            frame_delay -= 10;
                        }
                    } else {
                        // If paused, make one step forward
                        make_one_step(&mut coordinates, &velocities, max_x, max_y);
                        history.push(coordinates.clone()); // Save new state to history
                        step += 1; // Increment step counter
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    // If running, slow down animation
                    if running {
                        if frame_delay < 500 {
                            frame_delay += 10;
                        }
                    } else if step > 0 {
                        // If paused, make one step backward
                        coordinates = history.pop().unwrap(); // Restore the last state
                        step -= 1; // Decrement step counter
                    }
                }
                _ => {}
            }
        }

        if running {
            // Make a step when running
            make_one_step(&mut coordinates, &velocities, max_x, max_y);
            history.push(coordinates.clone()); // Save new state to history
            step += 1;
        }

        // Clear the canvas
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        // Draw the grid (optional)
        canvas.set_draw_color(Color::GRAY);
        for i in 0..=max_x {
            let pos_x = i as i32 * scale_x;
            canvas.draw_line((pos_x, 0), (pos_x, window_height as i32))?;
        }
        for i in 0..=max_y {
            let pos_y = i as i32 * scale_y;
            canvas.draw_line((0, pos_y), (window_width as i32, pos_y))?;
        }

        // Draw the updated coordinates
        canvas.set_draw_color(Color::RED);
        for &(x, y) in &coordinates {
            let rect = Rect::new(
                x * scale_x,
                y * scale_y,
                scale_x as u32,
                scale_y as u32,
            );
            canvas.fill_rect(rect)?; // Draw a filled rectangle for each point
        }

        // Render the step counter as text
        let surface = font
            .render(&format!("Step: {}", step))
            .blended(Color::WHITE)
            .map_err(|e| e.to_string())?;
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;
        let text_width = surface.width();
        let text_height = surface.height();

        // Draw the text in the top-right corner
        canvas.copy(
            &texture,
            None,
            Rect::new(
                (window_width - text_width as u32) as i32 - 10,
                10,
                text_width,
                text_height,
            ),
        )?;

        // Present the updated canvas
        canvas.present();

        // Control animation speed
        if running {
            std::thread::sleep(Duration::from_millis(frame_delay));
        }
    }

    Ok(())
}

// Function to make one step in the simulation
fn make_one_step(
    coordinates: &mut Vec<(i32, i32)>,
    velocities: &[(i32, i32)],
    max_x: i32,
    max_y: i32,
) {
    for (i, coord) in coordinates.iter_mut().enumerate() {
        coord.0 = ((coord.0 + velocities[i].0) % max_x).rem_euclid(max_x);
        coord.1 = ((coord.1 + velocities[i].1) % max_y).rem_euclid(max_y);
    }
}
