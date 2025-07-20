mod cpu;

use cpu::Cpu;
use sdl2::{self, event::Event, keyboard::Keycode};
use std::env;

const SCALE: u32 = 15;
const INSTRUCTIONS_PER_FRAME: usize = 10;

struct SquareWave{
    phase_func: f32,
    phase: f32,
    volume: f32,
}
impl sdl2::audio::AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_func) % 1.0;
        }
    }
}

fn draw_screen(gfx: &[u8; 64*32], canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    canvas.set_draw_color(sdl2::pixels::Color::RGB(24, 24, 37));
    canvas.clear();

    canvas.set_draw_color(sdl2::pixels::Color::RGB(205, 214, 244));
    for y in 0..32 {
        for x in 0..64 {
            if gfx[y * 64 + x] == 1 {
                let rect = sdl2::rect::Rect::new((x as u32 * SCALE) as i32, (y as u32 * SCALE) as i32, SCALE, SCALE);
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

    let audio = sdl_context.audio().unwrap();

    let desired_spec = sdl2::audio::AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1),
        samples: None,
    };
    
    let device = audio.open_playback(None, &desired_spec, |spec| {
        SquareWave {
            phase_func: 440.0 / spec.freq as f32,
            phase: 0.0,
            volume: 0.25,
        }
    }).unwrap();
    let mut sound_on = false;

    let mut cpu = Cpu::new();

    let rom_path = env::args().nth(1).unwrap_or("rom/pong.ch8".to_string());
    cpu.load_rom(&rom_path).unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => return,
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => return,
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
        
        for _ in 0..INSTRUCTIONS_PER_FRAME {
            cpu.cycle().unwrap();
        }

        if cpu.draw_flag {
            draw_screen(&cpu.gfx, &mut canvas);
            cpu.draw_flag = false;
        }

        if cpu.sound_timer > 0 {
            cpu.sound_timer -= 1;
            if !sound_on {
                device.resume();
                sound_on = true;
            }
        } else if sound_on {
            device.pause();
            sound_on = false;
        }
        if cpu.delay_timer > 0 {
            cpu.delay_timer -= 1;
        };

        sdl_context.timer().unwrap().delay(1000 / 60);
    };
}
