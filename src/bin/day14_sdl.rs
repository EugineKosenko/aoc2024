use std::{fs, env, io::{self, BufRead}};
use sdl2::pixels::Color;
use sdl2::{event::Event, keyboard::Keycode};
use sdl2::rect::Rect;
use std::time::Duration;

const WIDTH: u32 = 101;
const HEIGHT: u32 = 103;
const SCALE: u32 = 5;
const WIN_WIDTH: u32 = SCALE * WIDTH;
const WIN_HEIGHT: u32 = SCALE * HEIGHT;
const FONT: &'static str = "/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf";
const MAX_DELAY: u64 = 1000;
const STEP_DELAY: u64 = 10;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let lines = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap());
    lazy_static::lazy_static! {
        static ref RE_ROBOT: regex::Regex = regex::Regex::new(r"^p=(?P<px>-?\d+),(?P<py>-?\d+) v=(?P<vx>-?\d+),(?P<vy>-?\d+)$").unwrap();
    }
    let mut robots = lines
        .map(|line| {
            let cps = RE_ROBOT.captures(&line).unwrap();
            let px = cps.name("px").unwrap().as_str().parse::<i32>().unwrap();
            let py = cps.name("py").unwrap().as_str().parse::<i32>().unwrap();
            let vx = cps.name("vx").unwrap().as_str().parse::<i32>().unwrap();
            let vy = cps.name("vy").unwrap().as_str().parse::<i32>().unwrap();
            ((px, py), (vx, vy))
        })
        .collect::<Vec<_>>();
    let context = sdl2::init().unwrap();
    let mut canvas = context.video().unwrap()
        .window("Day 4, part 2", WIN_WIDTH, WIN_HEIGHT)
        .position_centered()
        .build().unwrap()
        .into_canvas().build().unwrap();
    let ttf = sdl2::ttf::init().unwrap();
    let font = ttf.load_font(FONT, 32).unwrap();
    let mut events = context.event_pump().unwrap();
    let mut step = 0;
    let mut dir = 1;
    let mut is_auto = false;
    let mut delay = MAX_DELAY;
    'running: loop {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        canvas.set_draw_color(Color::GRAY);
        for i in 0..=WIDTH {
            let x = (i * SCALE) as i32;
            canvas.draw_line((x, 0), (x, WIN_HEIGHT as i32)).unwrap();
        }
        
        for i in 0..=HEIGHT {
            let y = (i * SCALE) as i32;
            canvas.draw_line((0, y), (WIN_WIDTH as i32, y)).unwrap();
        }
        canvas.set_draw_color(Color::GREEN);
        for &((x, y), _) in &robots {
            canvas.fill_rect(Rect::new(x * SCALE as i32, y * SCALE as i32, SCALE as u32, SCALE as u32)).unwrap();
        }
        let surface = font
            .render(&format!("Step: {}", step))
            .blended(Color::WHITE).unwrap();
        let creator = canvas.texture_creator();
        let texture = creator
            .create_texture_from_surface(&surface).unwrap();
        canvas.copy(
            &texture, None,
            Rect::new((WIN_WIDTH - surface.width() as u32) as i32 - 10, 10, surface.width(), surface.height())).unwrap();
        canvas.present();
        for event in events.poll_iter() {
            match event {
                Event::Quit {..}
                | Event::KeyDown { keycode: Some(Keycode::Escape), ..} => { break 'running; },
                Event::KeyDown { keycode: Some(Keycode::Right), ..} => {
                    if !is_auto {
                        step += 1;
                        for ((ref mut x, ref mut y), (vx, vy)) in &mut robots {
                            *x = (*x + *vx).rem_euclid(WIDTH as i32);
                            *y = (*y + *vy).rem_euclid(HEIGHT as i32);
                        }
                    } else {
                        dir = 1;
                    }
                },
                
                Event::KeyDown { keycode: Some(Keycode::Left), ..} => {
                    if !is_auto {
                        step -= 1;
                        for ((ref mut x, ref mut y), (vx, vy)) in &mut robots {
                            *x = (*x - *vx).rem_euclid(WIDTH as i32);
                            *y = (*y - *vy).rem_euclid(HEIGHT as i32);
                        }
                    } else {
                        dir = -1;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Space), ..} => {
                    is_auto = !is_auto;
                },
                Event::KeyDown { keycode: Some(Keycode::Down), ..} => { if delay < MAX_DELAY { delay += STEP_DELAY; } },
                Event::KeyDown { keycode: Some(Keycode::Up), ..} => { if delay > 0 { delay -= STEP_DELAY; } }
                _ => { /* nothing */ }
            }
        }
        if is_auto {
            step += dir;
            for ((ref mut x, ref mut y), (vx, vy)) in &mut robots {
                *x = (*x + dir * *vx).rem_euclid(WIDTH as i32);
                *y = (*y + dir * *vy).rem_euclid(HEIGHT as i32);
            }
            std::thread::sleep(Duration::from_millis(delay));
        }
    }
}
