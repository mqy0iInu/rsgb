use std::env;
use std::path::PathBuf;

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate sdl2;

use std::thread;
use std::time;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;

mod cartridge;
mod cpu;
mod common;
mod gamepad;
mod mmu;
mod ppu;
mod timer;
mod cgb;

fn translate_keycode(key: Keycode) -> Option<gamepad::Key> {
    match key {
        Keycode::Down => Some(gamepad::Key::Down),
        Keycode::Up => Some(gamepad::Key::Up),
        Keycode::Left => Some(gamepad::Key::Left),
        Keycode::Right => Some(gamepad::Key::Right),
        Keycode::Return => Some(gamepad::Key::Start),
        Keycode::RShift => Some(gamepad::Key::Select),
        Keycode::X => Some(gamepad::Key::A),
        Keycode::Z => Some(gamepad::Key::B),
        _ => None,
    }
}

/// Handles key down event.
fn handle_keydown(cpu: &mut cpu::CPU, key: Keycode) {
    translate_keycode(key).map(|k| cpu.mmu.gamepad.keydown(k));
}

/// Handles key up event.
fn handle_keyup(cpu: &mut cpu::CPU, key: Keycode) {
    translate_keycode(key).map(|k| cpu.mmu.gamepad.keyup(k));
}

/// Returns ROM filename.
fn rom_fname() -> String {
    env::args().nth(1).unwrap()
}

/// Returns save filename for current ROM.
fn save_fname() -> String {
    let mut path_buf = PathBuf::from(rom_fname());
    path_buf.set_extension("sav");
    path_buf.to_str().unwrap().to_string()
}

fn main() {
    env_logger::init();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("RSGB -Rust GB Emu-", 320, 288)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, 160, 144)
        .unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut cpu = cpu::CPU::new(&rom_fname());
    cpu.mmu.cartridge.read_save_file(&save_fname());

    // CGB Unlock
    let cgb_flg = cpu.mmu.cartridge.get_cgb_mode();
    cpu.mmu.cgb.cgb_unlock(cgb_flg);

    'running: loop {
        let now = time::Instant::now();
        let mut elapsed_tick: u32 = 0;

        // Emulate one frame
        while elapsed_tick < 456 * (144 + 10) {
            elapsed_tick += cpu.step() as u32;
        }

        texture
            .with_lock(None, |buf: &mut [u8], pitch: usize| {
                let fb = cpu.mmu.ppu.frame_buffer();

                for y in 0..144 {
                    for x in 0..160 {
                        let offset = y * pitch + x * 3;
                        let color = fb[y * 160 + x];

                        buf[offset] = color;
                        buf[offset + 1] = color;
                        buf[offset + 2] = color;
                    }
                }
            })
            .unwrap();

        canvas.clear();
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => handle_keydown(&mut cpu, keycode),
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => handle_keyup(&mut cpu, keycode),
                _ => (),
            }
        }

        let wait = time::Duration::from_micros(1000000 / 60);
        let elapsed = now.elapsed();

        if wait > elapsed {
            thread::sleep(wait - elapsed);
        }
    }

    cpu.mmu.cartridge.write_save_file(&save_fname());
}
