mod state;
use state::State;

fn main() {
    let state = State::new(&[
        0x00,
        0x20,
        0x0A,
        0x10,
        0x11,
        0x30,
        0x11,
        0x40,
        0x00,
        0x50,
        0x11,
        0x60,
        0x70,
        0x11,
        0x80,
        0x12,
        0xFF,
        0x00,
        0x90,
        0x15,
        0xFF,
        0x00,
        0xA0,
        0x00,
        0xB0,
        0x1C,
        0xFF,
        0x00,
        0xC0,
        0x00,
        0xD0,
        0xFF,
    ]);
    let final_state = state.start();

    println!("--- FINAL MEMORY ---");
    final_state.print_memory(30);
}

