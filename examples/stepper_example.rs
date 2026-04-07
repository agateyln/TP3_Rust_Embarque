#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use panic_probe as _;
use tp3::bsp_ensea::Board;
use tp3::stepper::{Direction, MicrosteppingMode, Stepper};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let board = Board::new();
    let pins = board.stepper_pins;

    let mut stepper = Stepper::new(pins.dir, pins.step, pins.enable, pins.microstep1, pins.microstep2);

    stepper.enable();

    loop {
        // microstepping full
        stepper.set_microstepping(MicrosteppingMode::Full);
        info!("stepper: clockwise, microstepping=full");
        stepper.set_speed(1000, Direction::Clockwise);
        stepper.step_count(600).await;
        Timer::after(Duration::from_millis(300)).await;

        info!("stepper: counter-clockwise, microstepping=full");
        stepper.set_speed(1000, Direction::CounterClockwise);
        stepper.step_count(600).await;
        Timer::after(Duration::from_millis(300)).await;

        // microstepping half
        stepper.set_microstepping(MicrosteppingMode::Half);
        info!("stepper: clockwise, microstepping=half");
        stepper.set_speed(1000, Direction::Clockwise);
        stepper.step_count(600).await;
        Timer::after(Duration::from_millis(300)).await;

        info!("stepper: counter-clockwise, microstepping=half");
        stepper.set_speed(1000, Direction::CounterClockwise);
        stepper.step_count(600).await;
        Timer::after(Duration::from_millis(300)).await;

        // microstepping quarter
        stepper.set_microstepping(MicrosteppingMode::Quarter);
        info!("stepper: clockwise, microstepping=quarter");
        stepper.set_speed(1000, Direction::Clockwise);
        stepper.step_count(600).await;
        Timer::after(Duration::from_millis(300)).await;

        info!("stepper: counter-clockwise, microstepping=quarter");
        stepper.set_speed(1000, Direction::CounterClockwise);
        stepper.step_count(600).await;
        Timer::after(Duration::from_millis(300)).await;

        // microstepping eighth
        stepper.set_microstepping(MicrosteppingMode::Eighth);
        info!("stepper: clockwise, microstepping=eighth");
        stepper.set_speed(1000, Direction::Clockwise);
        stepper.step_count(600).await;
        Timer::after(Duration::from_millis(300)).await;

        info!("stepper: counter-clockwise, microstepping=eighth");
        stepper.set_speed(1000, Direction::CounterClockwise);
        stepper.step_count(600).await;
        Timer::after(Duration::from_millis(300)).await;
    }
}
