use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};
use embassy_stm32::peripherals;
use embassy_stm32::timer::qei::{Config as QeiConfig, Qei};

pub struct Board {
    pub bargraph_pins: BargraphPins,
    pub gamepad_pins: GamepadPins,
    pub encoder_pins: EncoderPins,
    pub stepper_pins: StepperPins,
}

pub struct BargraphPins {
    pub bargraph_0: Output<'static>,
    pub bargraph_1: Output<'static>,
    pub bargraph_2: Output<'static>,
    pub bargraph_3: Output<'static>,
    pub bargraph_4: Output<'static>,
    pub bargraph_5: Output<'static>,
    pub bargraph_6: Output<'static>,
    pub bargraph_7: Output<'static>,
}

pub struct GamepadPins {
    pub up: Input<'static>,
    pub down: Input<'static>,
    pub left: Input<'static>,
    pub right: Input<'static>,
    pub center: Input<'static>,
}

pub struct EncoderPins {
    pub qei: Qei<'static, peripherals::TIM2>,
    pub button: Input<'static>,
}

pub struct StepperPins {
    pub dir: Output<'static>,
    pub step: Output<'static>,
    pub enable: Output<'static>,
    pub microstep1: Output<'static>,
    pub microstep2: Output<'static>,
}

impl BargraphPins {
    pub fn into_array(self) -> [Output<'static>; 8] {
        [
            self.bargraph_0,
            self.bargraph_1,
            self.bargraph_2,
            self.bargraph_3,
            self.bargraph_4,
            self.bargraph_5,
            self.bargraph_6,
            self.bargraph_7,
        ]
    }
}

impl Board {
    pub fn new() -> Self {
        let p = embassy_stm32::init(Default::default());

        let bargraph_pins = BargraphPins {
            bargraph_0: Output::new(p.PB5, Level::Low, Speed::Low),
            bargraph_1: Output::new(p.PB14, Level::Low, Speed::Low),
            bargraph_2: Output::new(p.PB4, Level::Low, Speed::Low),
            bargraph_3: Output::new(p.PB15, Level::Low, Speed::Low),
            bargraph_4: Output::new(p.PB1, Level::Low, Speed::Low),
            bargraph_5: Output::new(p.PA8, Level::Low, Speed::Low),
            bargraph_6: Output::new(p.PB2, Level::Low, Speed::Low),
            bargraph_7: Output::new(p.PC7, Level::Low, Speed::Low),
        };

        let gamepad_pins = GamepadPins {
            up: Input::new(p.PC8, Pull::Up),
            down: Input::new(p.PB11, Pull::Up),
            left: Input::new(p.PC6, Pull::Up),
            right: Input::new(p.PC9, Pull::Up),
            center: Input::new(p.PC5, Pull::Up),
        };

        let encoder_pins = EncoderPins {
            qei: Qei::new(
                p.TIM2,
                p.PA0,
                p.PA1,
                QeiConfig {
                    ch1_pull: Pull::Up,
                    ch2_pull: Pull::Up,
                    ..Default::default()
                },
            ),
            button: Input::new(p.PA15, Pull::Up),
        };

        let stepper_pins = StepperPins {
            dir: Output::new(p.PA7, Level::Low, Speed::Low),
            step: Output::new(p.PA6, Level::Low, Speed::Low),
            enable: Output::new(p.PA12, Level::Low, Speed::Low),
            microstep1: Output::new(p.PA11, Level::Low, Speed::Low),
            microstep2: Output::new(p.PB12, Level::Low, Speed::Low),
        };

        Self {
            bargraph_pins,
            gamepad_pins,
            encoder_pins,
            stepper_pins,
        }
    }
}
