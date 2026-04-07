use core::convert::Infallible;

use embassy_stm32::pac;
use embassy_stm32::peripherals;
use embassy_stm32::timer::qei::{Direction, Qei};
use embedded_hal::digital::InputPin;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[derive(defmt::Format)]
pub enum EncoderDirection {
    Upcounting,
    Downcounting,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[derive(defmt::Format)]
pub struct EncoderState {
    pub position: i32,
    pub pressed: bool,
    pub direction: EncoderDirection,
}

pub struct Encoder<'d, BUTTON>
where
    BUTTON: InputPin<Error = Infallible>,
{
    qei: Qei<'d, peripherals::TIM2>,
    button: BUTTON,
}

impl<'d, BUTTON> Encoder<'d, BUTTON>
where
    BUTTON: InputPin<Error = Infallible>,
{
    const COUNTER_MAX: u32 = 10_000;

    pub fn new(qei: Qei<'d, peripherals::TIM2>, button: BUTTON) -> Self {
        let mut encoder = Self { qei, button };
        encoder.reset();
        encoder
    }

    pub fn position(&self) -> i32 {
        pac::TIM2.cnt().read() as i32
    }

    pub fn direction_state(&self) -> EncoderDirection {
        match self.qei.read_direction() {
            Direction::Upcounting => EncoderDirection::Upcounting,
            Direction::Downcounting => EncoderDirection::Downcounting,
        }
    }

    pub fn is_pressed(&mut self) -> bool {
        self.button.is_low().unwrap_or(false)
    }

    pub fn poll(&mut self) -> EncoderState {
        EncoderState {
            position: self.position(),
            pressed: self.is_pressed(),
            direction: self.direction_state(),
        }
    }

    pub fn set_position(&mut self, position: i32) {
        let tim2 = pac::TIM2;
        let clamped = position.clamp(0, Self::COUNTER_MAX as i32) as u32;
        tim2.arr().write_value(Self::COUNTER_MAX); 
        tim2.cnt().write_value(clamped);
    }

    pub fn reset(&mut self) {
        self.set_position(0);
    }
}
