//! PIT 8254 programmable interval timer - 100 Hz tick

use x86_64::instructions::port::Port;

const PIT_CHANNEL0: u16 = 0x40;
const PIT_COMMAND:  u16 = 0x43;
const PIT_FREQ: u32 = 1193182;

pub fn init_pit_hz(hz: u32) {
    let divisor = PIT_FREQ / hz;
    unsafe {
        let mut cmd: Port<u8> = Port::new(PIT_COMMAND);
        let mut ch0: Port<u8> = Port::new(PIT_CHANNEL0);
        cmd.write(0x36); // channel 0, lo/hi, mode 3 square wave
        ch0.write((divisor & 0xFF) as u8);
        ch0.write(((divisor >> 8) & 0xFF) as u8);
    }
    crate::println!("[PIT] Channel0 @ {} Hz (divisor {})", hz, divisor);
}

pub fn init_pit_100hz() { init_pit_hz(100); }
