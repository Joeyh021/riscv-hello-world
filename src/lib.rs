#![no_std]

mod serial;
use old_hal::blocking::delay::DelayMs;
use riscv::delay::McycleDelay;
use serial::Serial;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut serial = Serial::new();
    let mut delay = McycleDelay::new(100_000_000);
    serial.print("\n");
    loop {
        serial.println("Hello from Rust!");
        delay.delay_ms(100_u32);
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let mut serial = Serial::new();
    if let Some(s) = info.payload().downcast_ref::<&str>() {
        serial.print("panic occurred:");
        serial.print(s);
        serial.write_byte(b'\n');
    } else {
        serial.print("panic occurred");
    }
    loop {}
}
