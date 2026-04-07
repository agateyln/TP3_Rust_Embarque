use core::convert::Infallible;
use embassy_time::{Duration, Timer};
use embedded_hal::digital::OutputPin;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
	Clockwise,
	CounterClockwise,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MicrosteppingMode {
	Full, // 1 step per pulse 
	Half, // 2 steps per pulse
	Quarter, // 4 steps per pulse 
	Eighth, // 8 steps per pulse 
}

pub struct Stepper<DIR, STEP, ENABLE, MS1, MS2> {
	dir: DIR,
	step: STEP,
	enable: ENABLE,
	ms1: MS1,
	ms2: MS2,
	step_period_us: u32,
}

impl<DIR, STEP, ENABLE, MS1, MS2> Stepper<DIR, STEP, ENABLE, MS1, MS2>
where
	DIR: OutputPin<Error = Infallible>,
	STEP: OutputPin<Error = Infallible>,
	ENABLE: OutputPin<Error = Infallible>,
	MS1: OutputPin<Error = Infallible>,
	MS2: OutputPin<Error = Infallible>,
{
	pub fn new(dir: DIR, step: STEP, enable: ENABLE, ms1: MS1, ms2: MS2) -> Self {
		let mut driver = Self {
			dir,
			step,
			enable,
			ms1,
			ms2,
			step_period_us: 2_000,
		};

		driver.disable(); //désactiver le driver au démarrage 
		driver.set_microstepping(MicrosteppingMode::Full); //microstepping par défaut à full 
		driver // retourne le driver initialié
	}

	pub fn set_speed(&mut self, speed: u32, direction: Direction) {
		match direction {
			Direction::Clockwise => { // dir à l'état haut pour sens horaire
				let _ = self.dir.set_high(); 
			}
			Direction::CounterClockwise => { // dir à l'état bas pour sens anti-horaire
				let _ = self.dir.set_low();
			}
		}

		if speed == 0 { 
			self.step_period_us = u32::MAX; //valeur très grande pour ne pas faire de pas si vitesse nulle
			return; 
		}

		let period = 1_000_000u32 / speed; //période en us
		self.step_period_us = period.max(2); //limiter à 2us pour éviter vitesses trop grandes 
	}

	pub fn enable(&mut self) {
		let _ = self.enable.set_low();
	}

	pub fn disable(&mut self) {
		let _ = self.enable.set_high();
	}

	pub fn set_microstepping(&mut self, mode: MicrosteppingMode) {
		match mode {
			MicrosteppingMode::Full => {
				let _ = self.ms1.set_low();
				let _ = self.ms2.set_low();
			}
			MicrosteppingMode::Half => {
				let _ = self.ms1.set_high();
				let _ = self.ms2.set_low();
			}
			MicrosteppingMode::Quarter => { 
				let _ = self.ms1.set_low();
				let _ = self.ms2.set_high(); 
			}
			MicrosteppingMode::Eighth => {
				let _ = self.ms1.set_high();
				let _ = self.ms2.set_high();
			}
		}
	}

	pub async fn step_count(&mut self, steps: u32) {
		if self.step_period_us == u32::MAX { //si période trop grande, on ne fait pas de pas
			return;
		}

		let high_time = Duration::from_micros((self.step_period_us / 2).max(1) as u64); //high_time: moitié période d'au moins 1us
		let low_time = Duration::from_micros((self.step_period_us - (self.step_period_us / 2)).max(1) as u64);

		for _ in 0..steps { 
			let _ = self.step.set_high(); 
			Timer::after(high_time).await;
			let _ = self.step.set_low();
			Timer::after(low_time).await;
		}
	}
}



