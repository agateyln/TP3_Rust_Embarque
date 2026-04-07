use core::convert::{AsMut, AsRef};
use core::marker::PhantomData;
use embedded_hal::digital::{ErrorType, OutputPin};

#[derive(Debug)]
pub enum BargraphError<E> {
	InvalidRange,
	Pin(E),
}

pub struct Bargraph<PINS, PIN> {
	pins: PINS,
	min: i32,
	max: i32,
	_pin: PhantomData<PIN>,
}

impl<PINS, PIN> Bargraph<PINS, PIN>
where
	PINS: AsMut<[PIN]>,
	PIN: OutputPin + ErrorType,
{
	pub fn new(pins: PINS) -> Self {
		Self {
			pins,
			min: 0,
			max: 100,
			_pin: PhantomData,
		}
	}

	pub fn set_range(&mut self, min: i32, max: i32) -> Result<(), BargraphError<<PIN as ErrorType>::Error>> {
		if min >= max {
			return Err(BargraphError::InvalidRange);
		}

		self.min = min;
		self.max = max;
		Ok(())
	}

	pub fn set_value(&mut self, value: i32) -> Result<(), BargraphError<<PIN as ErrorType>::Error>> {
		let pins = self.pins.as_mut();
		let led_count = pins.len();

		if led_count == 0 {
			return Ok(());
		}

		let clamped = value.clamp(self.min, self.max);
		let range = (self.max - self.min) as i64;
		let relative = (clamped - self.min) as i64;

		let leds_on = ((relative * led_count as i64) / range) as usize;

		for (index, pin) in pins.iter_mut().enumerate() {
			if index < leds_on {
				pin.set_high().map_err(BargraphError::Pin)?;
			} else {
				pin.set_low().map_err(BargraphError::Pin)?;
			}
		}

		Ok(())
	}

	pub fn range(&self) -> (i32, i32) {
		(self.min, self.max)
	}

	pub fn pins(&self) -> &[PIN]
	where
		PINS: AsRef<[PIN]>,
	{
		self.pins.as_ref()
	}
}
