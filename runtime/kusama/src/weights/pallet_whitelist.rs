// Copyright 2017-2022 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.
//! Autogenerated weights for `pallet_whitelist`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-28, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm5`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("kusama-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=kusama-dev
// --steps=50
// --repeat=20
// --pallet=pallet_whitelist
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/kusama/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_whitelist`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_whitelist::WeightInfo for WeightInfo<T> {
	/// Storage: Whitelist WhitelistedCall (r:1 w:1)
	/// Proof: Whitelist WhitelistedCall (max_values: None, max_size: Some(40), added: 2515, mode: MaxEncodedLen)
	/// Storage: Preimage StatusFor (r:1 w:1)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	fn whitelist_call() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `151`
		//  Estimated: `5081`
		// Minimum execution time: 19_057 nanoseconds.
		Weight::from_ref_time(19_276_000)
			.saturating_add(Weight::from_proof_size(5081))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Whitelist WhitelistedCall (r:1 w:1)
	/// Proof: Whitelist WhitelistedCall (max_values: None, max_size: Some(40), added: 2515, mode: MaxEncodedLen)
	/// Storage: Preimage StatusFor (r:1 w:1)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	fn remove_whitelisted_call() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `280`
		//  Estimated: `5081`
		// Minimum execution time: 17_283 nanoseconds.
		Weight::from_ref_time(17_617_000)
			.saturating_add(Weight::from_proof_size(5081))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Whitelist WhitelistedCall (r:1 w:1)
	/// Proof: Whitelist WhitelistedCall (max_values: None, max_size: Some(40), added: 2515, mode: MaxEncodedLen)
	/// Storage: Preimage PreimageFor (r:1 w:1)
	/// Proof: Preimage PreimageFor (max_values: None, max_size: Some(4194344), added: 4196819, mode: Measured)
	/// Storage: Preimage StatusFor (r:1 w:1)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 4194294]`.
	fn dispatch_whitelisted_call(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `388 + n * (1 ±0)`
		//  Estimated: `7941 + n * (1 ±0)`
		// Minimum execution time: 28_589 nanoseconds.
		Weight::from_ref_time(28_824_000)
			.saturating_add(Weight::from_proof_size(7941))
			// Standard Error: 1
			.saturating_add(Weight::from_ref_time(1_175).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_proof_size(1).saturating_mul(n.into()))
	}
	/// Storage: Whitelist WhitelistedCall (r:1 w:1)
	/// Proof: Whitelist WhitelistedCall (max_values: None, max_size: Some(40), added: 2515, mode: MaxEncodedLen)
	/// Storage: Preimage StatusFor (r:1 w:1)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 10000]`.
	fn dispatch_whitelisted_call_with_preimage(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `280`
		//  Estimated: `5081`
		// Minimum execution time: 21_200 nanoseconds.
		Weight::from_ref_time(22_063_035)
			.saturating_add(Weight::from_proof_size(5081))
			// Standard Error: 4
			.saturating_add(Weight::from_ref_time(1_534).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
