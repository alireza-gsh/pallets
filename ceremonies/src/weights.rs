/*
Copyright 2022 Encointer

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

*/

//! Autogenerated weights for pallet_encointer_ceremonies with reference hardware:
//! * Core(TM) i7-10875H
//! * 32GB of RAM
//! * NVMe SSD
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-03-19, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/encointer-node-notee
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_encointer_ceremonies
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=runtime/src/weights/pallet_encointer_ceremonies.rs
// --template=/home/clang/code/encointer-node/scripts/frame-weight-template-full-info.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_encointer_ceremonies.
pub trait WeightInfo {
	fn register_participant() -> Weight;
	fn attest_claims() -> Weight;
	fn endorse_newcomer() -> Weight;
	fn claim_rewards() -> Weight;
	fn set_inactivity_timeout() -> Weight;
	fn set_meetup_time_offset() -> Weight;
	fn set_reputation_lifetime() -> Weight;
	fn set_endorsement_tickets_per_bootstrapper() -> Weight;
	fn set_time_tolerance() -> Weight;
	fn set_location_tolerance() -> Weight;
	fn purge_community_ceremony() -> Weight;
}

/// Weights for pallet_encointer_ceremonies using the Encointer solo chain node and recommended hardware.
pub struct EncointerWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for EncointerWeight<T> {
	fn register_participant() -> Weight {
		(369_200_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(13 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	fn attest_claims() -> Weight {
		(2_661_100_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(27 as Weight))
			.saturating_add(T::DbWeight::get().writes(12 as Weight))
	}
	fn endorse_newcomer() -> Weight {
		(124_100_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn claim_rewards() -> Weight {
		(1_680_100_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(62 as Weight))
			.saturating_add(T::DbWeight::get().writes(22 as Weight))
	}
	fn set_inactivity_timeout() -> Weight {
		(45_100_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_meetup_time_offset() -> Weight {
		(60_900_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_reputation_lifetime() -> Weight {
		(37_400_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_endorsement_tickets_per_bootstrapper() -> Weight {
		(36_300_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: EncointerCeremonies TimeTolerance (r:0 w:1)
	fn set_time_tolerance() -> Weight {
		(16_559_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: EncointerCeremonies LocationTolerance (r:0 w:1)
	fn set_location_tolerance() -> Weight {
		(15_876_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn purge_community_ceremony() -> Weight {
		(272_200_000 as Weight).saturating_add(T::DbWeight::get().writes(12 as Weight))
	}
}

// For tests
impl WeightInfo for () {
	fn register_participant() -> Weight {
		(369_200_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(13 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	fn attest_claims() -> Weight {
		(2_661_100_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(27 as Weight))
			.saturating_add(RocksDbWeight::get().writes(12 as Weight))
	}
	fn endorse_newcomer() -> Weight {
		(124_100_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(8 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	fn claim_rewards() -> Weight {
		(1_680_100_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(62 as Weight))
			.saturating_add(RocksDbWeight::get().writes(22 as Weight))
	}
	fn set_inactivity_timeout() -> Weight {
		(45_100_000 as Weight).saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn set_meetup_time_offset() -> Weight {
		(60_900_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn set_reputation_lifetime() -> Weight {
		(37_400_000 as Weight).saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn set_endorsement_tickets_per_bootstrapper() -> Weight {
		(36_300_000 as Weight).saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: EncointerCeremonies TimeTolerance (r:0 w:1)
	fn set_time_tolerance() -> Weight {
		(16_559_000 as Weight).saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: EncointerCeremonies LocationTolerance (r:0 w:1)
	fn set_location_tolerance() -> Weight {
		(15_876_000 as Weight).saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn purge_community_ceremony() -> Weight {
		(272_200_000 as Weight).saturating_add(RocksDbWeight::get().writes(12 as Weight))
	}
}
