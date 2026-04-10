use heapless::String;
use embassy_stm32::gpio::{Output};

use core::sync::atomic:: Ordering;


//Variables partagées
use crate::shared::{BARGRAPH_LEVEL,BARGRAPH_SIGNAL };

#[derive(Debug)]
pub enum BargraphError<E> {
	InvalidRange,
	Pin(E),
}

pub struct Bargraph<const N:usize> {
	pins: [Output<'static>;N],
	min: i32,
	max: i32,
}

impl<const N: usize> Bargraph<N> {
	pub fn new(pins: [Output<'static>; N]) -> Self {
		Self {
			pins,
			min: 0,
			max: 100,
		}
	}

	pub fn set_range(&mut self, min: i32, max: i32) -> Result<(), String<64>> {
		if min >= max {
			self.min=min;
			self.max=min;
		}

		self.min = min;
		self.max = max;
		Ok(())
	}

	pub fn set_value(&mut self, value: i32) -> Result<(), String<64>> {
		let led_count = self.pins.len();

		if led_count == 0 {
			return Ok(());
		}

		let clamped = value.clamp(self.min, self.max);
		let range = (self.max - self.min) as i64;
		let relative = (clamped - self.min) as i64;

		let leds_on = ((relative * led_count as i64) / range) as usize;

		for (index, pin) in self.pins.iter_mut().enumerate() {
			if index < leds_on {
				pin.set_high();
			} else {
				pin.set_low();
			}
		}

		Ok(())
	}

	pub fn range(&self) -> (i32, i32) {
		(self.min, self.max)
	}


	///Méthode asynchrone qui sera notifiée à chaque changement de valeur de BARGRAPH_SIGNAL.
	pub async fn wait_and_update( &mut self) -> Result<(),  String<64>> {
		BARGRAPH_SIGNAL.wait().await ;
		let value = BARGRAPH_LEVEL.load(Ordering::Relaxed) as i32; //Relaxed : seul la mémoir directement utilisée est synchronisé
		self.set_value(value)?;
		BARGRAPH_SIGNAL.reset();

		Ok(())

	}

	pub fn update_value(new_value : u32) -> Result<(), String<64>> {
		BARGRAPH_LEVEL.store(new_value, Ordering::Relaxed);
		BARGRAPH_SIGNAL.signal(()); //Notifie le signal
		Ok(())
	}



}

