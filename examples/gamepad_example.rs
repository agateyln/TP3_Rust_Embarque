#![no_std]
#![no_main]

use tp3::bsp_ensea::Board;
use tp3::gamepad::Gamepad;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use defmt::info;
use defmt_rtt as _;
use panic_probe as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let board = Board::new();

    let mut gamepad = Gamepad::new(
        board.gamepad_pins.up,
        board.gamepad_pins.down,
        board.gamepad_pins.left,
        board.gamepad_pins.right,
        board.gamepad_pins.center,
    );

    loop {
        let state = gamepad.poll();
        info!(
            "gamepad: up={} down={} left={} right={} center={}",
            state.up,
            state.down,
            state.left,
            state.right,
            state.center
        );
        Timer::after(Duration::from_millis(120)).await;
    }
}
