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
//! Autogenerated weights for `pallet_bridge_grandpa`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-03-22, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("polkadot-dev"), DB CACHE: 1024

// Executed Command:
// target/production/polkadot
// benchmark
// --chain=polkadot-dev
// --steps=50
// --repeat=20
// --pallet=pallet_bridge_grandpa
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --header=./file_header.txt
// --output=./runtime/polkadot/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_bridge_grandpa`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_bridge_grandpa::WeightInfo for WeightInfo<T> {
	// Storage: BridgeKusamaGrandpa IsHalted (r:1 w:0)
	// Storage: BridgeKusamaGrandpa RequestCount (r:1 w:1)
	// Storage: BridgeKusamaGrandpa BestFinalized (r:1 w:1)
	// Storage: BridgeKusamaGrandpa ImportedHeaders (r:1 w:2)
	// Storage: BridgeKusamaGrandpa CurrentAuthoritySet (r:1 w:0)
	// Storage: BridgeKusamaGrandpa ImportedHashesPointer (r:1 w:1)
	// Storage: BridgeKusamaGrandpa ImportedHashes (r:1 w:1)
	fn submit_finality_proof(p: u32, v: u32, ) -> Weight {
		(82_911_000 as Weight)
			// Standard Error: 7_000
			.saturating_add((46_952_000 as Weight).saturating_mul(p as Weight))
			// Standard Error: 7_000
			.saturating_add((1_396_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
}