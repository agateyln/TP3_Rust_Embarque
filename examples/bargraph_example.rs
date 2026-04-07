#![no_std]
#![no_main]

use tp3::bargraph::Bargraph;
use tp3::bsp_ensea::Board;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use defmt_rtt as _;
use panic_probe as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let board = Board::new();
    let mut bargraph = Bargraph::new(board.bargraph_pins.into_array());

    let _ = bargraph.set_range(10, 90);

    loop {
        for value in (0..=100).step_by(10) {
            let _ = bargraph.set_value(value);
            Timer::after(Duration::from_millis(120)).await;
        }

        for value in (0..=100).rev().step_by(10) {
            let _ = bargraph.set_value(value);
            Timer::after(Duration::from_millis(120)).await;
        }
    }
}
