// Copyright 2021 Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_balances`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-12-21, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm4`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("statemint-dev"), DB CACHE: 1024

// Executed Command:
// ./artifacts/polkadot-parachain
// benchmark
// pallet
// --chain=statemint-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_balances
// --extrinsic=*
// --steps=50
// --repeat=20
// --json
// --header=./file_header.txt
// --output=./parachains/runtimes/assets/statemint/src/weights/pallet_balances.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_balances`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_balances::WeightInfo for WeightInfo<T> {
	// Storage: System Account (r:1 w:1)
	fn transfer() -> Weight {
		// Minimum execution time: 42_089 nanoseconds.
		Weight::from_ref_time(42_634_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: System Account (r:1 w:1)
	fn transfer_keep_alive() -> Weight {
		// Minimum execution time: 32_299 nanoseconds.
		Weight::from_ref_time(32_672_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: System Account (r:1 w:1)
	fn set_balance_creating() -> Weight {
		// Minimum execution time: 23_721 nanoseconds.
		Weight::from_ref_time(24_222_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: System Account (r:1 w:1)
	fn set_balance_killing() -> Weight {
		// Minimum execution time: 26_112 nanoseconds.
		Weight::from_ref_time(26_748_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: System Account (r:2 w:2)
	fn force_transfer() -> Weight {
		// Minimum execution time: 41_276 nanoseconds.
		Weight::from_ref_time(41_815_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: System Account (r:1 w:1)
	fn transfer_all() -> Weight {
		// Minimum execution time: 37_021 nanoseconds.
		Weight::from_ref_time(37_426_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: System Account (r:1 w:1)
	fn force_unreserve() -> Weight {
		// Minimum execution time: 20_580 nanoseconds.
		Weight::from_ref_time(21_111_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
