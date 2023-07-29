// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for `pallet_broker`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-07-29, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner-ynta1nyy-project-145-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! EXECUTION: ``, WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// target/production/substrate-node
// benchmark
// pallet
// --steps=50
// --repeat=20
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --json-file=/builds/parity/mirrors/substrate/.git/.artifacts/bench.json
// --pallet=pallet_broker
// --chain=dev
// --header=./HEADER-APACHE2
// --output=./frame/broker/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_broker`.
pub trait WeightInfo {
	fn configure() -> Weight;
	fn reserve() -> Weight;
	fn unreserve() -> Weight;
	fn set_lease() -> Weight;
	fn start_sales(n: u32, ) -> Weight;
	fn purchase() -> Weight;
	fn renew() -> Weight;
	fn transfer() -> Weight;
	fn partition() -> Weight;
	fn interlace() -> Weight;
	fn assign() -> Weight;
	fn pool() -> Weight;
	fn claim_revenue(m: u32, ) -> Weight;
	fn purchase_credit() -> Weight;
	fn drop_region() -> Weight;
	fn drop_contribution() -> Weight;
	fn drop_history() -> Weight;
	fn drop_renewal() -> Weight;
	fn request_core_count(n: u32, ) -> Weight;
	fn process_core_count(n: u32, ) -> Weight;
	fn process_revenue() -> Weight;
	fn rotate_sale(n: u32, ) -> Weight;
	fn process_pool() -> Weight;
	fn process_core_schedule() -> Weight;
	fn request_revenue_info_at() -> Weight;
}

/// Weights for `pallet_broker` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `Broker::Configuration` (r:0 w:1)
	/// Proof: `Broker::Configuration` (`max_values`: Some(1), `max_size`: Some(31), added: 526, mode: `MaxEncodedLen`)
	fn configure() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_448_000 picoseconds.
		Weight::from_parts(3_729_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Broker::Reservations` (r:1 w:1)
	/// Proof: `Broker::Reservations` (`max_values`: Some(1), `max_size`: Some(6011), added: 6506, mode: `MaxEncodedLen`)
	fn reserve() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `5016`
		//  Estimated: `7496`
		// Minimum execution time: 22_537_000 picoseconds.
		Weight::from_parts(23_335_000, 7496)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Broker::Reservations` (r:1 w:1)
	/// Proof: `Broker::Reservations` (`max_values`: Some(1), `max_size`: Some(6011), added: 6506, mode: `MaxEncodedLen`)
	fn unreserve() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6218`
		//  Estimated: `7496`
		// Minimum execution time: 21_668_000 picoseconds.
		Weight::from_parts(22_442_000, 7496)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Broker::Leases` (r:1 w:1)
	/// Proof: `Broker::Leases` (`max_values`: Some(1), `max_size`: Some(41), added: 536, mode: `MaxEncodedLen`)
	fn set_lease() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `239`
		//  Estimated: `1526`
		// Minimum execution time: 13_606_000 picoseconds.
		Weight::from_parts(14_104_000, 1526)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Broker::Configuration` (r:1 w:0)
	/// Proof: `Broker::Configuration` (`max_values`: Some(1), `max_size`: Some(31), added: 526, mode: `MaxEncodedLen`)
	/// Storage: `Broker::InstaPoolIo` (r:3 w:3)
	/// Proof: `Broker::InstaPoolIo` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Reservations` (r:1 w:0)
	/// Proof: `Broker::Reservations` (`max_values`: Some(1), `max_size`: Some(6011), added: 6506, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Leases` (r:1 w:1)
	/// Proof: `Broker::Leases` (`max_values`: Some(1), `max_size`: Some(41), added: 536, mode: `MaxEncodedLen`)
	/// Storage: `Broker::SaleInfo` (r:0 w:1)
	/// Proof: `Broker::SaleInfo` (`max_values`: Some(1), `max_size`: Some(57), added: 552, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Status` (r:0 w:1)
	/// Proof: `Broker::Status` (`max_values`: Some(1), `max_size`: Some(18), added: 513, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Workplan` (r:0 w:10)
	/// Proof: `Broker::Workplan` (`max_values`: None, `max_size`: Some(1216), added: 3691, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[0, 1000]`.
	fn start_sales(_n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6330`
		//  Estimated: `8499`
		// Minimum execution time: 64_012_000 picoseconds.
		Weight::from_parts(67_819_922, 8499)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(16_u64))
	}
	/// Storage: `Broker::Status` (r:1 w:0)
	/// Proof: `Broker::Status` (`max_values`: Some(1), `max_size`: Some(18), added: 513, mode: `MaxEncodedLen`)
	/// Storage: `Broker::SaleInfo` (r:1 w:1)
	/// Proof: `Broker::SaleInfo` (`max_values`: Some(1), `max_size`: Some(57), added: 552, mode: `MaxEncodedLen`)
	/// Storage: `Authorship::Author` (r:1 w:0)
	/// Proof: `Authorship::Author` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	/// Storage: `System::Digest` (r:1 w:0)
	/// Proof: `System::Digest` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Broker::Regions` (r:0 w:1)
	/// Proof: `Broker::Regions` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	fn purchase() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `568`
		//  Estimated: `2053`
		// Minimum execution time: 48_110_000 picoseconds.
		Weight::from_parts(49_234_000, 2053)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Broker::Configuration` (r:1 w:0)
	/// Proof: `Broker::Configuration` (`max_values`: Some(1), `max_size`: Some(31), added: 526, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Status` (r:1 w:0)
	/// Proof: `Broker::Status` (`max_values`: Some(1), `max_size`: Some(18), added: 513, mode: `MaxEncodedLen`)
	/// Storage: `Broker::SaleInfo` (r:1 w:1)
	/// Proof: `Broker::SaleInfo` (`max_values`: Some(1), `max_size`: Some(57), added: 552, mode: `MaxEncodedLen`)
	/// Storage: `Broker::AllowedRenewals` (r:1 w:2)
	/// Proof: `Broker::AllowedRenewals` (`max_values`: None, `max_size`: Some(1233), added: 3708, mode: `MaxEncodedLen`)
	/// Storage: `Authorship::Author` (r:1 w:0)
	/// Proof: `Authorship::Author` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	/// Storage: `System::Digest` (r:1 w:0)
	/// Proof: `System::Digest` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Broker::Workplan` (r:0 w:1)
	/// Proof: `Broker::Workplan` (`max_values`: None, `max_size`: Some(1216), added: 3691, mode: `MaxEncodedLen`)
	fn renew() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `686`
		//  Estimated: `4698`
		// Minimum execution time: 69_580_000 picoseconds.
		Weight::from_parts(70_914_000, 4698)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `Broker::Regions` (r:1 w:1)
	/// Proof: `Broker::Regions` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	fn transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `495`
		//  Estimated: `3550`
		// Minimum execution time: 17_687_000 picoseconds.
		Weight::from_parts(18_573_000, 3550)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Broker::Regions` (r:1 w:2)
	/// Proof: `Broker::Regions` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	fn partition() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `495`
		//  Estimated: `3550`
		// Minimum execution time: 19_675_000 picoseconds.
		Weight::from_parts(20_234_000, 3550)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Broker::Regions` (r:1 w:2)
	/// Proof: `Broker::Regions` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	fn interlace() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `495`
		//  Estimated: `3550`
		// Minimum execution time: 19_426_000 picoseconds.
		Weight::from_parts(20_414_000, 3550)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Broker::Configuration` (r:1 w:0)
	/// Proof: `Broker::Configuration` (`max_values`: Some(1), `max_size`: Some(31), added: 526, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Status` (r:1 w:0)
	/// Proof: `Broker::Status` (`max_values`: Some(1), `max_size`: Some(18), added: 513, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Regions` (r:1 w:1)
	/// Proof: `Broker::Regions` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Workplan` (r:1 w:1)
	/// Proof: `Broker::Workplan` (`max_values`: None, `max_size`: Some(1216), added: 3691, mode: `MaxEncodedLen`)
	fn assign() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `740`
		//  Estimated: `4681`
		// Minimum execution time: 31_751_000 picoseconds.
		Weight::from_parts(32_966_000, 4681)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Broker::Status` (r:1 w:0)
	/// Proof: `Broker::Status` (`max_values`: Some(1), `max_size`: Some(18), added: 513, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Regions` (r:1 w:1)
	/// Proof: `Broker::Regions` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Workplan` (r:1 w:1)
	/// Proof: `Broker::Workplan` (`max_values`: None, `max_size`: Some(1216), added: 3691, mode: `MaxEncodedLen`)
	/// Storage: `Broker::InstaPoolIo` (r:2 w:2)
	/// Proof: `Broker::InstaPoolIo` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `Broker::InstaPoolContribution` (r:0 w:1)
	/// Proof: `Broker::InstaPoolContribution` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	fn pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `775`
		//  Estimated: `5996`
		// Minimum execution time: 36_709_000 picoseconds.
		Weight::from_parts(38_930_000, 5996)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: `Broker::InstaPoolContribution` (r:1 w:1)
	/// Proof: `Broker::InstaPoolContribution` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	/// Storage: `Broker::InstaPoolHistory` (r:3 w:1)
	/// Proof: `Broker::InstaPoolHistory` (`max_values`: None, `max_size`: Some(45), added: 2520, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `m` is `[1, 3]`.
	fn claim_revenue(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `720`
		//  Estimated: `6196 + m * (2520 ±0)`
		// Minimum execution time: 55_510_000 picoseconds.
		Weight::from_parts(56_665_061, 6196)
			// Standard Error: 61_729
			.saturating_add(Weight::from_parts(1_724_824, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(m.into())))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(Weight::from_parts(0, 2520).saturating_mul(m.into()))
	}
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn purchase_credit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `103`
		//  Estimated: `3593`
		// Minimum execution time: 44_992_000 picoseconds.
		Weight::from_parts(46_225_000, 3593)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Broker::Status` (r:1 w:0)
	/// Proof: `Broker::Status` (`max_values`: Some(1), `max_size`: Some(18), added: 513, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Regions` (r:1 w:1)
	/// Proof: `Broker::Regions` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	fn drop_region() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `603`
		//  Estimated: `3550`
		// Minimum execution time: 28_207_000 picoseconds.
		Weight::from_parts(28_707_000, 3550)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Broker::Configuration` (r:1 w:0)
	/// Proof: `Broker::Configuration` (`max_values`: Some(1), `max_size`: Some(31), added: 526, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Status` (r:1 w:0)
	/// Proof: `Broker::Status` (`max_values`: Some(1), `max_size`: Some(18), added: 513, mode: `MaxEncodedLen`)
	/// Storage: `Broker::InstaPoolContribution` (r:1 w:1)
	/// Proof: `Broker::InstaPoolContribution` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	fn drop_contribution() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `601`
		//  Estimated: `3533`
		// Minimum execution time: 31_813_000 picoseconds.
		Weight::from_parts(32_612_000, 3533)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Broker::Configuration` (r:1 w:0)
	/// Proof: `Broker::Configuration` (`max_values`: Some(1), `max_size`: Some(31), added: 526, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Status` (r:1 w:0)
	/// Proof: `Broker::Status` (`max_values`: Some(1), `max_size`: Some(18), added: 513, mode: `MaxEncodedLen`)
	/// Storage: `Broker::InstaPoolHistory` (r:1 w:1)
	/// Proof: `Broker::InstaPoolHistory` (`max_values`: None, `max_size`: Some(45), added: 2520, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn drop_history() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `829`
		//  Estimated: `3593`
		// Minimum execution time: 38_571_000 picoseconds.
		Weight::from_parts(39_493_000, 3593)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Broker::Status` (r:1 w:0)
	/// Proof: `Broker::Status` (`max_values`: Some(1), `max_size`: Some(18), added: 513, mode: `MaxEncodedLen`)
	/// Storage: `Broker::AllowedRenewals` (r:1 w:1)
	/// Proof: `Broker::AllowedRenewals` (`max_values`: None, `max_size`: Some(1233), added: 3708, mode: `MaxEncodedLen`)
	fn drop_renewal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `525`
		//  Estimated: `4698`
		// Minimum execution time: 24_714_000 picoseconds.
		Weight::from_parts(25_288_000, 4698)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: UNKNOWN KEY `0x18194fcb5c1fcace44d2d0a004272614` (r:0 w:1)
	/// Proof: UNKNOWN KEY `0x18194fcb5c1fcace44d2d0a004272614` (r:0 w:1)
	/// The range of component `n` is `[0, 1000]`.
	fn request_core_count(_n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 7_258_000 picoseconds.
		Weight::from_parts(7_925_570, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: UNKNOWN KEY `0x18194fcb5c1fcace44d2d0a004272614` (r:1 w:0)
	/// Proof: UNKNOWN KEY `0x18194fcb5c1fcace44d2d0a004272614` (r:1 w:0)
	/// The range of component `n` is `[0, 1000]`.
	fn process_core_count(_n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `97`
		//  Estimated: `3562`
		// Minimum execution time: 7_136_000 picoseconds.
		Weight::from_parts(7_788_194, 3562)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: `Broker::InstaPoolHistory` (r:0 w:1)
	/// Proof: `Broker::InstaPoolHistory` (`max_values`: None, `max_size`: Some(45), added: 2520, mode: `MaxEncodedLen`)
	fn process_revenue() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_049_000 picoseconds.
		Weight::from_parts(6_311_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Broker::InstaPoolIo` (r:3 w:3)
	/// Proof: `Broker::InstaPoolIo` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Reservations` (r:1 w:0)
	/// Proof: `Broker::Reservations` (`max_values`: Some(1), `max_size`: Some(6011), added: 6506, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Leases` (r:1 w:1)
	/// Proof: `Broker::Leases` (`max_values`: Some(1), `max_size`: Some(41), added: 536, mode: `MaxEncodedLen`)
	/// Storage: `Broker::SaleInfo` (r:0 w:1)
	/// Proof: `Broker::SaleInfo` (`max_values`: Some(1), `max_size`: Some(57), added: 552, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Workplan` (r:0 w:10)
	/// Proof: `Broker::Workplan` (`max_values`: None, `max_size`: Some(1216), added: 3691, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[0, 1000]`.
	fn rotate_sale(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6281`
		//  Estimated: `8499`
		// Minimum execution time: 47_504_000 picoseconds.
		Weight::from_parts(49_778_098, 8499)
			// Standard Error: 109
			.saturating_add(Weight::from_parts(427, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(15_u64))
	}
	/// Storage: `Broker::InstaPoolIo` (r:1 w:0)
	/// Proof: `Broker::InstaPoolIo` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `Broker::InstaPoolHistory` (r:0 w:1)
	/// Proof: `Broker::InstaPoolHistory` (`max_values`: None, `max_size`: Some(45), added: 2520, mode: `MaxEncodedLen`)
	fn process_pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `180`
		//  Estimated: `3493`
		// Minimum execution time: 9_573_000 picoseconds.
		Weight::from_parts(10_034_000, 3493)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Broker::Workplan` (r:1 w:1)
	/// Proof: `Broker::Workplan` (`max_values`: None, `max_size`: Some(1216), added: 3691, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Workload` (r:1 w:1)
	/// Proof: `Broker::Workload` (`max_values`: None, `max_size`: Some(1212), added: 3687, mode: `MaxEncodedLen`)
	fn process_core_schedule() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1423`
		//  Estimated: `4681`
		// Minimum execution time: 21_331_000 picoseconds.
		Weight::from_parts(22_235_000, 4681)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	fn request_revenue_info_at() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 191_000 picoseconds.
		Weight::from_parts(234_000, 0)
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `Broker::Configuration` (r:0 w:1)
	/// Proof: `Broker::Configuration` (`max_values`: Some(1), `max_size`: Some(31), added: 526, mode: `MaxEncodedLen`)
	fn configure() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_448_000 picoseconds.
		Weight::from_parts(3_729_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Broker::Reservations` (r:1 w:1)
	/// Proof: `Broker::Reservations` (`max_values`: Some(1), `max_size`: Some(6011), added: 6506, mode: `MaxEncodedLen`)
	fn reserve() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `5016`
		//  Estimated: `7496`
		// Minimum execution time: 22_537_000 picoseconds.
		Weight::from_parts(23_335_000, 7496)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Broker::Reservations` (r:1 w:1)
	/// Proof: `Broker::Reservations` (`max_values`: Some(1), `max_size`: Some(6011), added: 6506, mode: `MaxEncodedLen`)
	fn unreserve() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6218`
		//  Estimated: `7496`
		// Minimum execution time: 21_668_000 picoseconds.
		Weight::from_parts(22_442_000, 7496)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Broker::Leases` (r:1 w:1)
	/// Proof: `Broker::Leases` (`max_values`: Some(1), `max_size`: Some(41), added: 536, mode: `MaxEncodedLen`)
	fn set_lease() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `239`
		//  Estimated: `1526`
		// Minimum execution time: 13_606_000 picoseconds.
		Weight::from_parts(14_104_000, 1526)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Broker::Configuration` (r:1 w:0)
	/// Proof: `Broker::Configuration` (`max_values`: Some(1), `max_size`: Some(31), added: 526, mode: `MaxEncodedLen`)
	/// Storage: `Broker::InstaPoolIo` (r:3 w:3)
	/// Proof: `Broker::InstaPoolIo` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Reservations` (r:1 w:0)
	/// Proof: `Broker::Reservations` (`max_values`: Some(1), `max_size`: Some(6011), added: 6506, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Leases` (r:1 w:1)
	/// Proof: `Broker::Leases` (`max_values`: Some(1), `max_size`: Some(41), added: 536, mode: `MaxEncodedLen`)
	/// Storage: `Broker::SaleInfo` (r:0 w:1)
	/// Proof: `Broker::SaleInfo` (`max_values`: Some(1), `max_size`: Some(57), added: 552, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Status` (r:0 w:1)
	/// Proof: `Broker::Status` (`max_values`: Some(1), `max_size`: Some(18), added: 513, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Workplan` (r:0 w:10)
	/// Proof: `Broker::Workplan` (`max_values`: None, `max_size`: Some(1216), added: 3691, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[0, 1000]`.
	fn start_sales(_n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6330`
		//  Estimated: `8499`
		// Minimum execution time: 64_012_000 picoseconds.
		Weight::from_parts(67_819_922, 8499)
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(16_u64))
	}
	/// Storage: `Broker::Status` (r:1 w:0)
	/// Proof: `Broker::Status` (`max_values`: Some(1), `max_size`: Some(18), added: 513, mode: `MaxEncodedLen`)
	/// Storage: `Broker::SaleInfo` (r:1 w:1)
	/// Proof: `Broker::SaleInfo` (`max_values`: Some(1), `max_size`: Some(57), added: 552, mode: `MaxEncodedLen`)
	/// Storage: `Authorship::Author` (r:1 w:0)
	/// Proof: `Authorship::Author` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	/// Storage: `System::Digest` (r:1 w:0)
	/// Proof: `System::Digest` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Broker::Regions` (r:0 w:1)
	/// Proof: `Broker::Regions` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	fn purchase() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `568`
		//  Estimated: `2053`
		// Minimum execution time: 48_110_000 picoseconds.
		Weight::from_parts(49_234_000, 2053)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `Broker::Configuration` (r:1 w:0)
	/// Proof: `Broker::Configuration` (`max_values`: Some(1), `max_size`: Some(31), added: 526, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Status` (r:1 w:0)
	/// Proof: `Broker::Status` (`max_values`: Some(1), `max_size`: Some(18), added: 513, mode: `MaxEncodedLen`)
	/// Storage: `Broker::SaleInfo` (r:1 w:1)
	/// Proof: `Broker::SaleInfo` (`max_values`: Some(1), `max_size`: Some(57), added: 552, mode: `MaxEncodedLen`)
	/// Storage: `Broker::AllowedRenewals` (r:1 w:2)
	/// Proof: `Broker::AllowedRenewals` (`max_values`: None, `max_size`: Some(1233), added: 3708, mode: `MaxEncodedLen`)
	/// Storage: `Authorship::Author` (r:1 w:0)
	/// Proof: `Authorship::Author` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	/// Storage: `System::Digest` (r:1 w:0)
	/// Proof: `System::Digest` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Broker::Workplan` (r:0 w:1)
	/// Proof: `Broker::Workplan` (`max_values`: None, `max_size`: Some(1216), added: 3691, mode: `MaxEncodedLen`)
	fn renew() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `686`
		//  Estimated: `4698`
		// Minimum execution time: 69_580_000 picoseconds.
		Weight::from_parts(70_914_000, 4698)
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: `Broker::Regions` (r:1 w:1)
	/// Proof: `Broker::Regions` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	fn transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `495`
		//  Estimated: `3550`
		// Minimum execution time: 17_687_000 picoseconds.
		Weight::from_parts(18_573_000, 3550)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Broker::Regions` (r:1 w:2)
	/// Proof: `Broker::Regions` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	fn partition() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `495`
		//  Estimated: `3550`
		// Minimum execution time: 19_675_000 picoseconds.
		Weight::from_parts(20_234_000, 3550)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `Broker::Regions` (r:1 w:2)
	/// Proof: `Broker::Regions` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	fn interlace() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `495`
		//  Estimated: `3550`
		// Minimum execution time: 19_426_000 picoseconds.
		Weight::from_parts(20_414_000, 3550)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `Broker::Configuration` (r:1 w:0)
	/// Proof: `Broker::Configuration` (`max_values`: Some(1), `max_size`: Some(31), added: 526, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Status` (r:1 w:0)
	/// Proof: `Broker::Status` (`max_values`: Some(1), `max_size`: Some(18), added: 513, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Regions` (r:1 w:1)
	/// Proof: `Broker::Regions` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Workplan` (r:1 w:1)
	/// Proof: `Broker::Workplan` (`max_values`: None, `max_size`: Some(1216), added: 3691, mode: `MaxEncodedLen`)
	fn assign() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `740`
		//  Estimated: `4681`
		// Minimum execution time: 31_751_000 picoseconds.
		Weight::from_parts(32_966_000, 4681)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `Broker::Status` (r:1 w:0)
	/// Proof: `Broker::Status` (`max_values`: Some(1), `max_size`: Some(18), added: 513, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Regions` (r:1 w:1)
	/// Proof: `Broker::Regions` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Workplan` (r:1 w:1)
	/// Proof: `Broker::Workplan` (`max_values`: None, `max_size`: Some(1216), added: 3691, mode: `MaxEncodedLen`)
	/// Storage: `Broker::InstaPoolIo` (r:2 w:2)
	/// Proof: `Broker::InstaPoolIo` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `Broker::InstaPoolContribution` (r:0 w:1)
	/// Proof: `Broker::InstaPoolContribution` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	fn pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `775`
		//  Estimated: `5996`
		// Minimum execution time: 36_709_000 picoseconds.
		Weight::from_parts(38_930_000, 5996)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	/// Storage: `Broker::InstaPoolContribution` (r:1 w:1)
	/// Proof: `Broker::InstaPoolContribution` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	/// Storage: `Broker::InstaPoolHistory` (r:3 w:1)
	/// Proof: `Broker::InstaPoolHistory` (`max_values`: None, `max_size`: Some(45), added: 2520, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `m` is `[1, 3]`.
	fn claim_revenue(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `720`
		//  Estimated: `6196 + m * (2520 ±0)`
		// Minimum execution time: 55_510_000 picoseconds.
		Weight::from_parts(56_665_061, 6196)
			// Standard Error: 61_729
			.saturating_add(Weight::from_parts(1_724_824, 0).saturating_mul(m.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(m.into())))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
			.saturating_add(Weight::from_parts(0, 2520).saturating_mul(m.into()))
	}
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn purchase_credit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `103`
		//  Estimated: `3593`
		// Minimum execution time: 44_992_000 picoseconds.
		Weight::from_parts(46_225_000, 3593)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Broker::Status` (r:1 w:0)
	/// Proof: `Broker::Status` (`max_values`: Some(1), `max_size`: Some(18), added: 513, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Regions` (r:1 w:1)
	/// Proof: `Broker::Regions` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	fn drop_region() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `603`
		//  Estimated: `3550`
		// Minimum execution time: 28_207_000 picoseconds.
		Weight::from_parts(28_707_000, 3550)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Broker::Configuration` (r:1 w:0)
	/// Proof: `Broker::Configuration` (`max_values`: Some(1), `max_size`: Some(31), added: 526, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Status` (r:1 w:0)
	/// Proof: `Broker::Status` (`max_values`: Some(1), `max_size`: Some(18), added: 513, mode: `MaxEncodedLen`)
	/// Storage: `Broker::InstaPoolContribution` (r:1 w:1)
	/// Proof: `Broker::InstaPoolContribution` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	fn drop_contribution() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `601`
		//  Estimated: `3533`
		// Minimum execution time: 31_813_000 picoseconds.
		Weight::from_parts(32_612_000, 3533)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Broker::Configuration` (r:1 w:0)
	/// Proof: `Broker::Configuration` (`max_values`: Some(1), `max_size`: Some(31), added: 526, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Status` (r:1 w:0)
	/// Proof: `Broker::Status` (`max_values`: Some(1), `max_size`: Some(18), added: 513, mode: `MaxEncodedLen`)
	/// Storage: `Broker::InstaPoolHistory` (r:1 w:1)
	/// Proof: `Broker::InstaPoolHistory` (`max_values`: None, `max_size`: Some(45), added: 2520, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn drop_history() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `829`
		//  Estimated: `3593`
		// Minimum execution time: 38_571_000 picoseconds.
		Weight::from_parts(39_493_000, 3593)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Broker::Status` (r:1 w:0)
	/// Proof: `Broker::Status` (`max_values`: Some(1), `max_size`: Some(18), added: 513, mode: `MaxEncodedLen`)
	/// Storage: `Broker::AllowedRenewals` (r:1 w:1)
	/// Proof: `Broker::AllowedRenewals` (`max_values`: None, `max_size`: Some(1233), added: 3708, mode: `MaxEncodedLen`)
	fn drop_renewal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `525`
		//  Estimated: `4698`
		// Minimum execution time: 24_714_000 picoseconds.
		Weight::from_parts(25_288_000, 4698)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: UNKNOWN KEY `0x18194fcb5c1fcace44d2d0a004272614` (r:0 w:1)
	/// Proof: UNKNOWN KEY `0x18194fcb5c1fcace44d2d0a004272614` (r:0 w:1)
	/// The range of component `n` is `[0, 1000]`.
	fn request_core_count(_n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 7_258_000 picoseconds.
		Weight::from_parts(7_925_570, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: UNKNOWN KEY `0x18194fcb5c1fcace44d2d0a004272614` (r:1 w:0)
	/// Proof: UNKNOWN KEY `0x18194fcb5c1fcace44d2d0a004272614` (r:1 w:0)
	/// The range of component `n` is `[0, 1000]`.
	fn process_core_count(_n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `97`
		//  Estimated: `3562`
		// Minimum execution time: 7_136_000 picoseconds.
		Weight::from_parts(7_788_194, 3562)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	/// Storage: `Broker::InstaPoolHistory` (r:0 w:1)
	/// Proof: `Broker::InstaPoolHistory` (`max_values`: None, `max_size`: Some(45), added: 2520, mode: `MaxEncodedLen`)
	fn process_revenue() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_049_000 picoseconds.
		Weight::from_parts(6_311_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Broker::InstaPoolIo` (r:3 w:3)
	/// Proof: `Broker::InstaPoolIo` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Reservations` (r:1 w:0)
	/// Proof: `Broker::Reservations` (`max_values`: Some(1), `max_size`: Some(6011), added: 6506, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Leases` (r:1 w:1)
	/// Proof: `Broker::Leases` (`max_values`: Some(1), `max_size`: Some(41), added: 536, mode: `MaxEncodedLen`)
	/// Storage: `Broker::SaleInfo` (r:0 w:1)
	/// Proof: `Broker::SaleInfo` (`max_values`: Some(1), `max_size`: Some(57), added: 552, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Workplan` (r:0 w:10)
	/// Proof: `Broker::Workplan` (`max_values`: None, `max_size`: Some(1216), added: 3691, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[0, 1000]`.
	fn rotate_sale(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6281`
		//  Estimated: `8499`
		// Minimum execution time: 47_504_000 picoseconds.
		Weight::from_parts(49_778_098, 8499)
			// Standard Error: 109
			.saturating_add(Weight::from_parts(427, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(15_u64))
	}
	/// Storage: `Broker::InstaPoolIo` (r:1 w:0)
	/// Proof: `Broker::InstaPoolIo` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `Broker::InstaPoolHistory` (r:0 w:1)
	/// Proof: `Broker::InstaPoolHistory` (`max_values`: None, `max_size`: Some(45), added: 2520, mode: `MaxEncodedLen`)
	fn process_pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `180`
		//  Estimated: `3493`
		// Minimum execution time: 9_573_000 picoseconds.
		Weight::from_parts(10_034_000, 3493)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Broker::Workplan` (r:1 w:1)
	/// Proof: `Broker::Workplan` (`max_values`: None, `max_size`: Some(1216), added: 3691, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Workload` (r:1 w:1)
	/// Proof: `Broker::Workload` (`max_values`: None, `max_size`: Some(1212), added: 3687, mode: `MaxEncodedLen`)
	fn process_core_schedule() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1423`
		//  Estimated: `4681`
		// Minimum execution time: 21_331_000 picoseconds.
		Weight::from_parts(22_235_000, 4681)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	fn request_revenue_info_at() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 191_000 picoseconds.
		Weight::from_parts(234_000, 0)
	}
}
