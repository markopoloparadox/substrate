//! An example pallet built purely with the `frame` crate.

#![cfg_attr(not(feature = "std"), no_std)]
use frame::prelude::*;

#[frame::pallet]
pub mod pallet {
	use super::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: IsType<<Self as frame_system::Config>::RuntimeEvent> + From<Event<Self>>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::event]
	pub enum Event<T: Config> {}
}

#[cfg(test)]
mod tests {
	use super::pallet as pallet_example;
	use frame::{prelude::*, testing_prelude::*};

	type UncheckedExtrinsic = MockUncheckedExtrinsic<Test>;
	type Block = MockBlock<Test>;

	construct_runtime!(
		pub enum Test
		where
			Block = Block,
			NodeBlock = Block,
			UncheckedExtrinsic = UncheckedExtrinsic,
			{
				System: frame_system,
				Example: pallet_example,
			}
	);

	#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
	impl frame_system::Config for Test {
		type BaseCallFilter = frame::traits::Everything;
		type RuntimeOrigin = RuntimeOrigin;
		type RuntimeCall = RuntimeCall;
		type RuntimeEvent = RuntimeEvent;
		type PalletInfo = PalletInfo;
		type OnSetCode = ();
	}

	impl pallet_example::Config for Test {
		type RuntimeEvent = RuntimeEvent;
	}
}
