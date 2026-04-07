#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use panic_probe as _;
use tp3::bsp_ensea::Board;
use tp3::encoder::Encoder;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let board = Board::new();
    let encoder_pins = board.encoder_pins;

    let mut encoder = Encoder::new(encoder_pins.qei, encoder_pins.button);
    encoder.set_position(5_000);

    loop {
        let state = encoder.poll();
        info!( 
            "encoder: position={} pressed={} direction={:?}",
            state.position,
            state.pressed,
            state.direction
        );
        Timer::after(Duration::from_millis(100)).await;
    }
}
