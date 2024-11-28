fn main() {
    let opcode:u16;
    let memory:[u8;4096];
    let v:[u8;16]; // cpu registers
    let I:u16; // index register
    let pc:u16;// program counter
    let gfx:[u8;64*32]; // array of pixel state for graphic
    let delay_timer:u8;
    let sound_timer:u8;
    // The system’s buzzer sounds whenever the sound timer reaches zero.

    let stack:[u8;16];
    let stack_pointer:u8;
    let key:[u8;16]; // keypad 
    

}
