// Copyright Parity Technologies (UK) Ltd.
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

//! Autogenerated weights for `pallet_xcm_benchmarks::generic`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-14, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm3`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("asset-hub-kusama-dev"), DB CACHE: 1024

// Executed Command:
// target/production/polkadot-parachain
// benchmark
// pallet
// --steps=50
// --repeat=20
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --json-file=/var/lib/gitlab-runner/builds/zyw4fam_/0/parity/mirrors/cumulus/.git/.artifacts/bench.json
// --pallet=pallet_xcm_benchmarks::generic
// --chain=asset-hub-kusama-dev
// --header=./file_header.txt
// --template=./templates/xcm-bench-template.hbs
// --output=./parachains/runtimes/assets/asset-hub-kusama/src/weights/xcm/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weights for `pallet_xcm_benchmarks::generic`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo<T> {
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Proof: ParachainInfo ParachainId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: unknown `0x3a696e747261626c6f636b5f656e74726f7079` (r:1 w:1)
	// Proof Skipped: unknown `0x3a696e747261626c6f636b5f656e74726f7079` (r:1 w:1)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Proof Skipped: PolkadotXcm SupportedVersion (max_values: None, max_size: None, mode: Measured)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Proof Skipped: PolkadotXcm VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Proof Skipped: PolkadotXcm SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Proof Skipped: ParachainSystem HostConfiguration (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	// Proof Skipped: ParachainSystem PendingUpwardMessages (max_values: Some(1), max_size: None, mode: Measured)
	pub fn report_holding() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `75`
		//  Estimated: `3540`
		// Minimum execution time: 347_943_000 picoseconds.
		Weight::from_parts(348_788_000, 3540)
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	pub fn buy_execution() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_881_000 picoseconds.
		Weight::from_parts(3_982_000, 0)
	}
	// Storage: PolkadotXcm Queries (r:1 w:0)
	// Proof Skipped: PolkadotXcm Queries (max_values: None, max_size: None, mode: Measured)
	pub fn query_response() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `69`
		//  Estimated: `3534`
		// Minimum execution time: 11_038_000 picoseconds.
		Weight::from_parts(11_244_000, 3534)
			.saturating_add(T::DbWeight::get().reads(1))
	}
	pub fn transact() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 13_223_000 picoseconds.
		Weight::from_parts(13_610_000, 0)
	}
	pub fn refund_surplus() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_202_000 picoseconds.
		Weight::from_parts(4_333_000, 0)
	}
	pub fn set_error_handler() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_794_000 picoseconds.
		Weight::from_parts(2_860_000, 0)
	}
	pub fn set_appendix() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_717_000 picoseconds.
		Weight::from_parts(2_787_000, 0)
	}
	pub fn clear_error() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_722_000 picoseconds.
		Weight::from_parts(2_818_000, 0)
	}
	pub fn descend_origin() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_664_000 picoseconds.
		Weight::from_parts(3_775_000, 0)
	}
	pub fn clear_origin() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_703_000 picoseconds.
		Weight::from_parts(2_792_000, 0)
	}
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Proof: ParachainInfo ParachainId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: unknown `0x3a696e747261626c6f636b5f656e74726f7079` (r:1 w:1)
	// Proof Skipped: unknown `0x3a696e747261626c6f636b5f656e74726f7079` (r:1 w:1)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Proof Skipped: PolkadotXcm SupportedVersion (max_values: None, max_size: None, mode: Measured)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Proof Skipped: PolkadotXcm VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Proof Skipped: PolkadotXcm SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Proof Skipped: ParachainSystem HostConfiguration (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	// Proof Skipped: ParachainSystem PendingUpwardMessages (max_values: Some(1), max_size: None, mode: Measured)
	pub fn report_error() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `75`
		//  Estimated: `3540`
		// Minimum execution time: 27_482_000 picoseconds.
		Weight::from_parts(28_077_000, 3540)
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: PolkadotXcm AssetTraps (r:1 w:1)
	// Proof Skipped: PolkadotXcm AssetTraps (max_values: None, max_size: None, mode: Measured)
	pub fn claim_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `126`
		//  Estimated: `3591`
		// Minimum execution time: 15_603_000 picoseconds.
		Weight::from_parts(15_899_000, 3591)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	pub fn trap() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_679_000 picoseconds.
		Weight::from_parts(2_755_000, 0)
	}
	// Storage: PolkadotXcm VersionNotifyTargets (r:1 w:1)
	// Proof Skipped: PolkadotXcm VersionNotifyTargets (max_values: None, max_size: None, mode: Measured)
	// Storage: unknown `0x3a696e747261626c6f636b5f656e74726f7079` (r:1 w:1)
	// Proof Skipped: unknown `0x3a696e747261626c6f636b5f656e74726f7079` (r:1 w:1)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Proof Skipped: PolkadotXcm SupportedVersion (max_values: None, max_size: None, mode: Measured)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Proof Skipped: PolkadotXcm VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Proof Skipped: PolkadotXcm SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Proof Skipped: ParachainSystem HostConfiguration (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	// Proof Skipped: ParachainSystem PendingUpwardMessages (max_values: Some(1), max_size: None, mode: Measured)
	pub fn subscribe_version() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `75`
		//  Estimated: `3540`
		// Minimum execution time: 30_136_000 picoseconds.
		Weight::from_parts(30_688_000, 3540)
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: PolkadotXcm VersionNotifyTargets (r:0 w:1)
	// Proof Skipped: PolkadotXcm VersionNotifyTargets (max_values: None, max_size: None, mode: Measured)
	pub fn unsubscribe_version() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_817_000 picoseconds.
		Weight::from_parts(4_978_000, 0)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Proof: ParachainInfo ParachainId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: unknown `0x3a696e747261626c6f636b5f656e74726f7079` (r:1 w:1)
	// Proof Skipped: unknown `0x3a696e747261626c6f636b5f656e74726f7079` (r:1 w:1)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Proof Skipped: PolkadotXcm SupportedVersion (max_values: None, max_size: None, mode: Measured)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Proof Skipped: PolkadotXcm VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Proof Skipped: PolkadotXcm SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Proof Skipped: ParachainSystem HostConfiguration (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	// Proof Skipped: ParachainSystem PendingUpwardMessages (max_values: Some(1), max_size: None, mode: Measured)
	pub fn initiate_reserve_withdraw() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `75`
		//  Estimated: `3540`
		// Minimum execution time: 387_992_000 picoseconds.
		Weight::from_parts(392_283_000, 3540)
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	pub fn burn_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 118_538_000 picoseconds.
		Weight::from_parts(121_013_000, 0)
	}
	pub fn expect_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 12_087_000 picoseconds.
		Weight::from_parts(12_313_000, 0)
	}
	pub fn expect_origin() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_784_000 picoseconds.
		Weight::from_parts(2_840_000, 0)
	}
	pub fn expect_error() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_693_000 picoseconds.
		Weight::from_parts(2_781_000, 0)
	}
	pub fn expect_transact_status() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_913_000 picoseconds.
		Weight::from_parts(2_979_000, 0)
	}
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Proof: ParachainInfo ParachainId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: unknown `0x3a696e747261626c6f636b5f656e74726f7079` (r:1 w:1)
	// Proof Skipped: unknown `0x3a696e747261626c6f636b5f656e74726f7079` (r:1 w:1)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Proof Skipped: PolkadotXcm SupportedVersion (max_values: None, max_size: None, mode: Measured)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Proof Skipped: PolkadotXcm VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Proof Skipped: PolkadotXcm SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Proof Skipped: ParachainSystem HostConfiguration (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	// Proof Skipped: ParachainSystem PendingUpwardMessages (max_values: Some(1), max_size: None, mode: Measured)
	pub fn query_pallet() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `75`
		//  Estimated: `3540`
		// Minimum execution time: 31_021_000 picoseconds.
		Weight::from_parts(31_786_000, 3540)
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	pub fn expect_pallet() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_201_000 picoseconds.
		Weight::from_parts(5_335_000, 0)
	}
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Proof: ParachainInfo ParachainId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: unknown `0x3a696e747261626c6f636b5f656e74726f7079` (r:1 w:1)
	// Proof Skipped: unknown `0x3a696e747261626c6f636b5f656e74726f7079` (r:1 w:1)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Proof Skipped: PolkadotXcm SupportedVersion (max_values: None, max_size: None, mode: Measured)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Proof Skipped: PolkadotXcm VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Proof Skipped: PolkadotXcm SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Proof Skipped: ParachainSystem HostConfiguration (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	// Proof Skipped: ParachainSystem PendingUpwardMessages (max_values: Some(1), max_size: None, mode: Measured)
	pub fn report_transact_status() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `75`
		//  Estimated: `3540`
		// Minimum execution time: 27_837_000 picoseconds.
		Weight::from_parts(28_360_000, 3540)
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	pub fn clear_transact_status() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_727_000 picoseconds.
		Weight::from_parts(2_817_000, 0)
	}
	pub fn set_topic() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_659_000 picoseconds.
		Weight::from_parts(2_701_000, 0)
	}
	pub fn clear_topic() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_714_000 picoseconds.
		Weight::from_parts(2_806_000, 0)
	}
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Proof: ParachainInfo ParachainId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	pub fn universal_origin() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `1489`
		// Minimum execution time: 5_104_000 picoseconds.
		Weight::from_parts(5_236_000, 1489)
			.saturating_add(T::DbWeight::get().reads(1))
	}
	pub fn set_fees_mode() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_694_000 picoseconds.
		Weight::from_parts(2_747_000, 0)
	}
	pub fn unpaid_execution() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_937_000 picoseconds.
		Weight::from_parts(2_992_000, 0)
	}
}
