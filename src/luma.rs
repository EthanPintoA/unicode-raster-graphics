#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Luma {
	pub luma: u8,
}

impl Luma {
	#[inline(always)]
	pub const fn new(luma: u8) -> Self {
		Self { luma }
	}
}

impl From<u8> for Luma {
	#[inline(always)]
	fn from(luma: u8) -> Self {
		Self { luma }
	}
}

impl From<Luma> for u8 {
	#[inline(always)]
	fn from(luma: Luma) -> Self {
		luma.luma
	}
}
