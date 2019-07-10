// Copyright 2015-2019 Parity Technologies (UK) Ltd.
// This file is part of Parity Ethereum.

// Parity Ethereum is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Ethereum is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Ethereum.  If not, see <http://www.gnu.org/licenses/>.

//! A generic verifier trait.

use call_contract::CallContract;
use client_traits::{BlockInfo, VerifyingEngine};
//use engines::Engine;
use crate::{
//	error::Error,
	verification::FullFamilyParams,
};
use common_types::{
	errors::EthcoreError,
	header::Header,
};
//use super::verification;

/// Should be used to verify blocks.
pub trait Verifier<C>: Send + Sync
	where C: BlockInfo + CallContract
{
	/// Verify a block relative to its parent and uncles.
	fn verify_block_family(
		&self,
		header: &Header,
		parent: &Header,
		engine: &dyn VerifyingEngine,
		do_full: Option<FullFamilyParams<C>>
	) -> Result<(), EthcoreError>;

	/// Do a final verification check for an enacted header vs its expected counterpart.
	fn verify_block_final(&self, expected: &Header, got: &Header) -> Result<(), EthcoreError>;
	/// Verify a block, inspecting external state.
	fn verify_block_external(&self, header: &Header, engine: &dyn VerifyingEngine) -> Result<(), EthcoreError>;
}