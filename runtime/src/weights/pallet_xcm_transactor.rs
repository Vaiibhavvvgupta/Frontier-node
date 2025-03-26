
//! Autogenerated weights for `pallet_xcm_transactor`
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
// --json-file=benchmarking/results/results-pallet_xcm_transactor.json
// --pallet=pallet_xcm_transactor
// --chain=dev
// --output=benchmarking/new-benchmarks/pallet_xcm_transactor.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_xcm_transactor`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_xcm_transactor::WeightInfo for WeightInfo<T> {
	/// Storage: `XcmTransactor::IndexToAccount` (r:1 w:1)
	/// Proof: `XcmTransactor::IndexToAccount` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn register() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `147`
		//  Estimated: `3612`
		// Minimum execution time: 13_670_000 picoseconds.
		Weight::from_parts(13_922_000, 0)
			.saturating_add(Weight::from_parts(0, 3612))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `XcmTransactor::IndexToAccount` (r:0 w:1)
	/// Proof: `XcmTransactor::IndexToAccount` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn deregister() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 8_355_000 picoseconds.
		Weight::from_parts(8_499_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `XcmTransactor::TransactInfoWithWeightLimit` (r:0 w:1)
	/// Proof: `XcmTransactor::TransactInfoWithWeightLimit` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn set_transact_info() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 9_668_000 picoseconds.
		Weight::from_parts(9_867_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `XcmTransactor::TransactInfoWithWeightLimit` (r:0 w:1)
	/// Proof: `XcmTransactor::TransactInfoWithWeightLimit` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn remove_transact_info() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 9_175_000 picoseconds.
		Weight::from_parts(9_388_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `XcmTransactor::DestinationAssetFeePerSecond` (r:0 w:1)
	/// Proof: `XcmTransactor::DestinationAssetFeePerSecond` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn set_fee_per_second() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 8_898_000 picoseconds.
		Weight::from_parts(9_200_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `AssetManager::AssetIdType` (r:1 w:0)
	/// Proof: `AssetManager::AssetIdType` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `XcmTransactor::IndexToAccount` (r:1 w:0)
	/// Proof: `XcmTransactor::IndexToAccount` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `XcmTransactor::RelayIndices` (r:1 w:0)
	/// Proof: `XcmTransactor::RelayIndices` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `XcmTransactor::TransactInfoWithWeightLimit` (r:1 w:0)
	/// Proof: `XcmTransactor::TransactInfoWithWeightLimit` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `XcmTransactor::DestinationAssetFeePerSecond` (r:1 w:0)
	/// Proof: `XcmTransactor::DestinationAssetFeePerSecond` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	/// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `AssetManager::AssetTypeId` (r:1 w:0)
	/// Proof: `AssetManager::AssetTypeId` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Assets::Asset` (r:1 w:0)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(174), added: 2649, mode: `MaxEncodedLen`)
	fn transact_through_derivative() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `625`
		//  Estimated: `4090`
		// Minimum execution time: 37_230_000 picoseconds.
		Weight::from_parts(38_023_000, 0)
			.saturating_add(Weight::from_parts(0, 4090))
			.saturating_add(T::DbWeight::get().reads(8))
	}
	/// Storage: `AssetManager::AssetIdType` (r:1 w:0)
	/// Proof: `AssetManager::AssetIdType` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `XcmTransactor::TransactInfoWithWeightLimit` (r:1 w:0)
	/// Proof: `XcmTransactor::TransactInfoWithWeightLimit` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `XcmTransactor::DestinationAssetFeePerSecond` (r:1 w:0)
	/// Proof: `XcmTransactor::DestinationAssetFeePerSecond` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	/// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `AssetManager::AssetTypeId` (r:1 w:0)
	/// Proof: `AssetManager::AssetTypeId` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Assets::Asset` (r:1 w:0)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(174), added: 2649, mode: `MaxEncodedLen`)
	fn transact_through_sovereign() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `559`
		//  Estimated: `4024`
		// Minimum execution time: 31_176_000 picoseconds.
		Weight::from_parts(32_106_000, 0)
			.saturating_add(Weight::from_parts(0, 4024))
			.saturating_add(T::DbWeight::get().reads(6))
	}
	/// Storage: `AssetManager::AssetIdType` (r:1 w:0)
	/// Proof: `AssetManager::AssetIdType` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `XcmTransactor::TransactInfoWithWeightLimit` (r:1 w:0)
	/// Proof: `XcmTransactor::TransactInfoWithWeightLimit` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `XcmTransactor::DestinationAssetFeePerSecond` (r:1 w:0)
	/// Proof: `XcmTransactor::DestinationAssetFeePerSecond` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	/// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `ParachainSystem::HostConfiguration` (r:1 w:0)
	/// Proof: `ParachainSystem::HostConfiguration` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::PendingUpwardMessages` (r:1 w:1)
	/// Proof: `ParachainSystem::PendingUpwardMessages` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn transact_through_signed() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `534`
		//  Estimated: `3999`
		// Minimum execution time: 45_757_000 picoseconds.
		Weight::from_parts(46_849_000, 0)
			.saturating_add(Weight::from_parts(0, 3999))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `XcmTransactor::RelayIndices` (r:1 w:0)
	/// Proof: `XcmTransactor::RelayIndices` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `AssetManager::AssetIdType` (r:1 w:0)
	/// Proof: `AssetManager::AssetIdType` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `XcmTransactor::TransactInfoWithWeightLimit` (r:1 w:0)
	/// Proof: `XcmTransactor::TransactInfoWithWeightLimit` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `XcmTransactor::DestinationAssetFeePerSecond` (r:1 w:0)
	/// Proof: `XcmTransactor::DestinationAssetFeePerSecond` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	/// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `ParachainSystem::HostConfiguration` (r:1 w:0)
	/// Proof: `ParachainSystem::HostConfiguration` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::PendingUpwardMessages` (r:1 w:1)
	/// Proof: `ParachainSystem::PendingUpwardMessages` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn hrmp_manage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `538`
		//  Estimated: `4003`
		// Minimum execution time: 49_550_000 picoseconds.
		Weight::from_parts(50_518_000, 0)
			.saturating_add(Weight::from_parts(0, 4003))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
