
//! Autogenerated weights for `pallet_balances`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 42.0.0
//! DATE: 2024-11-03, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-172-31-15-118`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: 1024

// Executed Command:
// target/release/evm-template-node
// benchmark
// pallet
// --steps=50
// --repeat=20
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --json-file=benchmarking/results/results-pallet_balances.json
// --pallet=pallet_balances
// --chain=dev
// --output=benchmarking/new-benchmarks/pallet_balances.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_balances`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_balances::WeightInfo for WeightInfo<T> {
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	fn transfer_allow_death() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `40`
		//  Estimated: `3581`
		// Minimum execution time: 62_495_000 picoseconds.
		Weight::from_parts(63_189_000, 0)
			.saturating_add(Weight::from_parts(0, 3581))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	fn transfer_keep_alive() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `40`
		//  Estimated: `3581`
		// Minimum execution time: 53_065_000 picoseconds.
		Weight::from_parts(53_612_000, 0)
			.saturating_add(Weight::from_parts(0, 3581))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	fn force_set_balance_creating() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `128`
		//  Estimated: `3581`
		// Minimum execution time: 20_586_000 picoseconds.
		Weight::from_parts(21_126_000, 0)
			.saturating_add(Weight::from_parts(0, 3581))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	fn force_set_balance_killing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `128`
		//  Estimated: `3581`
		// Minimum execution time: 28_230_000 picoseconds.
		Weight::from_parts(28_755_000, 0)
			.saturating_add(Weight::from_parts(0, 3581))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	fn force_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `168`
		//  Estimated: `6172`
		// Minimum execution time: 66_173_000 picoseconds.
		Weight::from_parts(67_063_000, 0)
			.saturating_add(Weight::from_parts(0, 6172))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	fn transfer_all() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `40`
		//  Estimated: `3581`
		// Minimum execution time: 66_332_000 picoseconds.
		Weight::from_parts(66_876_000, 0)
			.saturating_add(Weight::from_parts(0, 3581))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	fn force_unreserve() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `128`
		//  Estimated: `3581`
		// Minimum execution time: 24_584_000 picoseconds.
		Weight::from_parts(25_216_000, 0)
			.saturating_add(Weight::from_parts(0, 3581))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `System::Account` (r:999 w:999)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// The range of component `u` is `[1, 1000]`.
	fn upgrade_accounts(u: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + u * (124 ±0)`
		//  Estimated: `990 + u * (2591 ±0)`
		// Minimum execution time: 22_236_000 picoseconds.
		Weight::from_parts(22_690_000, 0)
			.saturating_add(Weight::from_parts(0, 990))
			// Standard Error: 10_861
			.saturating_add(Weight::from_parts(17_673_400, 0).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(u.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(u.into())))
			.saturating_add(Weight::from_parts(0, 2591).saturating_mul(u.into()))
	}
	fn force_adjust_total_issuance() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 7_715_000 picoseconds.
		Weight::from_parts(7_995_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	fn burn_allow_death() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 38_078_000 picoseconds.
		Weight::from_parts(38_549_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	fn burn_keep_alive() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 28_229_000 picoseconds.
		Weight::from_parts(28_727_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
}
