// We make sure this pallet uses `no_std` for compiling to Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_core::{
	sr25519::{Public, Signature},
	ByteArray, H256, H512,
};
use sp_runtime::traits::{BlakeTwo256, Hash, SaturatedConversion};
use sp_std::{collections::btree_map::BTreeMap, vec::Vec};

use super::{block_author::BlockAuthor, issuance::Issuance};

pub use pallet::*;

#[frame_support::pallet(dev_mode)]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	use super::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {}

	#[pallet::pallet]
	pub struct Pallet<T>(_);
}
