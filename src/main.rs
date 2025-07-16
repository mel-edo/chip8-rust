mod cpu;

use cpu::Cpu;

fn main() {
    let mut cpu = Cpu::new();

    Cpu::load_rom(&mut cpu, "../c8games/PONG");
}
