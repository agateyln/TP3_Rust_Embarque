#![no_std]
#![no_main]

use tp3::bargraph::Bargraph;
use tp3::bsp_ensea::Board;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use tp3::bargraph_task::bargraph_task;
use defmt_rtt as _;
use panic_probe as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {

    let board = Board::new();
    let bargraph_array = board.bargraph_pins.into_array();
    let mut bargraph = Bargraph::new(bargraph_array);

    let _ = bargraph.set_range(0, 100);

    _spawner.spawn(bargraph_task(bargraph)).unwrap();

    loop {
        for value in (0..=100).step_by(10) {
            Bargraph::<8>::update_value(value).ok(); // simple, pas de générique
            Timer::after(Duration::from_millis(200)).await;
        }
        for value in (0..=100).rev().step_by(10) {
            Bargraph::<8>::update_value(value).ok();
            Timer::after(Duration::from_millis(800)).await;
        }
    }
}
