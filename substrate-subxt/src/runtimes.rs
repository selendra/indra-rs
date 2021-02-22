// Copyright 2019-2020 Parity Technologies (UK) Ltd.
// This file is part of substrate-subxt.
//
// subxt is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// subxt is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with substrate-subxt.  If not, see <http://www.gnu.org/licenses/>.

use codec::Encode;
use sp_runtime::{
    generic::Header,
    impl_opaque_keys,
    traits::{BlakeTwo256, IdentifyAccount, Verify},
    MultiSignature, OpaqueExtrinsic,
};
use sp_std::prelude::*;

/// Application specific crypto types
///
/// # Note
///
/// These are redefined here to avoid dependencies on the substrate creates where they are defined.
/// They must be identical to the definitions in the target substrate version.
pub mod app {
    use application_crypto::{app_crypto, ed25519, key_types, sr25519};

    /// Authority discovery app crypto types
    pub mod authority_discovery {
        use super::*;
        app_crypto!(sr25519, key_types::AUTHORITY_DISCOVERY);
    }
    /// Babe app crypto types
    pub mod babe {
        use super::*;
        app_crypto!(sr25519, key_types::BABE);
    }
    /// Im online discovery app crypto types
    pub mod im_online {
        use super::*;
        app_crypto!(ed25519, key_types::IM_ONLINE);
    }
    /// Grandpa app crypto types
    pub mod grandpa {
        use super::*;
        app_crypto!(ed25519, key_types::GRANDPA);
    }
    /// Validator app crypto types
    pub mod validator {
        use super::*;
        app_crypto!(ed25519, sp_core::crypto::KeyTypeId(*b"para"));
    }

    /// Assignment app crypto types
    pub mod para_assignment {
        use super::*;
        app_crypto!(ed25519, sp_core::crypto::KeyTypeId(*b"asgn"));
    }
}
/// BABE marker struct
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Babe;
impl sp_runtime::BoundToRuntimeAppPublic for Babe {
    type Public = app::babe::Public;
}

/// ImOnline marker struct
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ImOnline;
impl sp_runtime::BoundToRuntimeAppPublic for ImOnline {
    type Public = app::im_online::Public;
}

/// GRANDPA marker struct
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Grandpa;
impl sp_runtime::BoundToRuntimeAppPublic for Grandpa {
    type Public = app::grandpa::Public;
}

/// Parachain marker struct
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Parachains;
impl sp_runtime::BoundToRuntimeAppPublic for Parachains {
    type Public = app::validator::Public;
}

/// Parachain marker struct
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ParaAssignment;
impl sp_runtime::BoundToRuntimeAppPublic for ParaAssignment {
    type Public = app::para_assignment::Public;
}

/// Authority discovery marker struct
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AuthorityDiscovery;
impl sp_runtime::BoundToRuntimeAppPublic for AuthorityDiscovery {
    type Public = app::authority_discovery::Public;
}

impl_opaque_keys! {
    /// Polkadot/Kusama runtime keys
    pub struct SessionKeys {
        /// GRANDPA session key
        pub grandpa: Grandpa,
        /// BABE session key
        pub babe: Babe,
        /// ImOnline session key
        pub im_online: ImOnline,
        /// ParachainValidator session key
        pub parachain_validator: Parachains,
        /// Parachainassignment session key
        pub para_assignment: ParaAssignment,
        /// AuthorityDiscovery session key
        pub authority_discovery: AuthorityDiscovery,
    }
}

use crate::{
    extrinsic::{DefaultExtra, SignedExtra},
    frame::{
        balances::{AccountData, Balances},
        contracts::Contracts,
        session::Session,
        staking::Staking,
        sudo::Sudo,
        system::System,
    },
};

/// Runtime trait.
pub trait Runtime: System + Sized + Send + Sync + 'static {
    /// Signature type.
    type Signature: Verify + Encode + Send + Sync + 'static;
    /// Transaction extras.
    type Extra: SignedExtra<Self> + Send + Sync + 'static;
}

/// Concrete type definitions compatible with those for kusama, v0.7
///
/// # Note
///
/// Main difference is `type Address = AccountId`.
/// Also the contracts module is not part of the kusama runtime.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IndracoreRuntime;

impl Runtime for IndracoreRuntime {
    type Signature = MultiSignature;
    type Extra = DefaultExtra<Self>;
}

impl System for IndracoreRuntime {
    type Index = u32;
    type BlockNumber = u32;
    type Hash = sp_core::H256;
    type Hashing = BlakeTwo256;
    type AccountId = <<MultiSignature as Verify>::Signer as IdentifyAccount>::AccountId;
    type Address = Self::AccountId;
    type Header = Header<Self::BlockNumber, BlakeTwo256>;
    type Extrinsic = OpaqueExtrinsic;
    type AccountData = AccountData<<Self as Balances>::Balance>;
}

impl Session for IndracoreRuntime {
    type ValidatorId = <Self as System>::AccountId;
    type Keys = SessionKeys;
}

impl Staking for IndracoreRuntime {}

impl Balances for IndracoreRuntime {
    type Balance = u128;
}

impl Sudo for IndracoreRuntime {}

impl Contracts for IndracoreRuntime {}
