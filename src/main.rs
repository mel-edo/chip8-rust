mod cpu;

use cpu::Cpu;
use sdl2::{self, event::Event, keyboard::Keycode};
use std::time::Duration;

const SCALE: u32 = 10;

fn draw_screen(gfx: &[u8; 64*32], canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    canvas.set_draw_color(sdl2::pixels::Color::BLACK);
    canvas.clear();

    canvas.set_draw_color(sdl2::pixels::Color::WHITE);
    for y in 0..32 {
        for x in 0..64 {
            if gfx[y * 64 + x] == 1 {
                let rect = sdl2::rect::Rect::new(x as i32, y as i32, SCALE, SCALE);
                canvas.fill_rect(rect).unwrap();
            }
        }
    }
    canvas.present();
}

fn map_key(key: Keycode) -> Option<usize> {
    match key {
        Keycode::Num1 => Some(0x1),
        Keycode::Num2 => Some(0x2),
        Keycode::Num3 => Some(0x3),
        Keycode::Num4 => Some(0xC),
        Keycode::Q => Some(0x4),
        Keycode::W => Some(0x5),
        Keycode::E => Some(0x6),
        Keycode::R => Some(0xD),
        Keycode::A => Some(0x7),
        Keycode::S => Some(0x8),
        Keycode::D => Some(0x9),
        Keycode::F => Some(0xE),
        Keycode::Z => Some(0xA),
        Keycode::X => Some(0x0),
        Keycode::C => Some(0xB),
        Keycode::V => Some(0xF),    
        _ => None,
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let mut canvas = video.window("Chip 8", 64 * SCALE, 32 * SCALE)
        .position_centered()
        .build().unwrap()
        .into_canvas().build().unwrap();

    let mut cpu = Cpu::new();

    cpu.load_rom("c8games/PONG").unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => return,
                Event::KeyUp { keycode: Some(k), .. } => {
                    if let Some(idx) = map_key(k) {
                        cpu.keypad[idx] = 0;
                    }
                },
                Event::KeyDown { keycode: Some(k), .. } => {
                    if let Some(idx) = map_key(k) {
                        cpu.keypad[idx] = 1;
                    }
                },
                _ => {},
            }
        }
        cpu.cycle().unwrap();

        if cpu.draw_flag {
            draw_screen(&cpu.gfx, &mut canvas);
            cpu.draw_flag = false;
        }

        std::thread::sleep(Duration::from_millis(1000 / 60));
    };
}
