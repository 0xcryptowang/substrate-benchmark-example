
//! Autogenerated weights for pallet_template
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-01-29, STEPS: `[20, ]`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// ./target/release/node-template
// benchmark
// --chain
// dev
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// pallet_template
// --extrinsic
// do_something
// --steps
// 20
// --repeat
// 10
// --raw
// --output
// ./pallets/template/src/weights.rs


#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for pallet_template.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_template::WeightInfo for WeightInfo<T> {
	fn do_something(_s: u32, ) -> Weight {
		(21_264_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}
