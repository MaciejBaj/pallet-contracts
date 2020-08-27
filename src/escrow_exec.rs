// Copyright 2018-2020 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate. If not, see <http://www.gnu.org/licenses/>.

use crate::{
	CodeHash, Config, ContractAddressFor, Event, RawEvent, Trait,
	TrieId, BalanceOf, ContractInfo, TrieIdGenerator,
	gas::{Gas, GasMeter, Token}, rent, storage, Error, ContractInfoOf
};
use bitflags::bitflags;
use sp_std::prelude::*;
use sp_runtime::traits::{Bounded, Zero, Convert, Saturating};
use frame_support::{
	dispatch::DispatchError,
	traits::{ExistenceRequirement, Currency, Time, Randomness},
	weights::Weight,
	ensure, StorageMap,
};
use std::{cell::RefCell, collections::HashMap, marker::PhantomData, rc::Rc};

use crate::exec::*;
use crate::exec::{TransferCause};

fn escrow_transfer<'a, T: Trait>(
	gas_meter: &mut GasMeter<T>,
	cause: TransferCause,
	origin: TransactorKind,
	transactor: &T::AccountId,
	dest: &T::AccountId,
	value: BalanceOf<T>,
	config: &'a Config<T>,
) -> Result<(), DispatchError> {
	println!("escrow_transfer");
	// ToDo: Here switch dest into Escrow + Make a relevant entry in the Escrow storage transfers.
	Ok(())
}

pub struct EscrowCallContext<'a, 'b: 'a, T: Trait + 'b, V: Vm<T> + 'b, L: Loader<T>> {
	pub config: &'a Config<T>,
	pub caller: T::AccountId,
	pub value_transferred: BalanceOf<T>,
	pub timestamp: MomentOf<T>,
	pub block_number: T::BlockNumber,
	pub call_context: CallContext<'a, 'b, T, V, L>
}

impl<'a, 'b: 'a, T, E, V, L> Ext for EscrowCallContext<'a, 'b, T, V, L>
	where
		T: Trait + 'b,
		V: Vm<T, Executable = E>,
		L: Loader<T, Executable = E>,
{
	type T = T;

	fn get_storage(&self, key: &StorageKey) -> Option<Vec<u8>> {
		self.call_context.get_storage(key)
	}

	fn set_storage(&mut self, key: StorageKey, value: Option<Vec<u8>>) {
		self.call_context.set_storage(key, value);
	}

	fn instantiate(
		&mut self,
		code_hash: &CodeHash<T>,
		endowment: BalanceOf<T>,
		gas_meter: &mut GasMeter<T>,
		input_data: Vec<u8>,
	) -> Result<(AccountIdOf<T>, ExecReturnValue), ExecError> {
		self.call_context.instantiate(code_hash, endowment, gas_meter, input_data)
	}

	fn transfer(
		&mut self,
		to: &T::AccountId,
		value: BalanceOf<T>,
		gas_meter: &mut GasMeter<T>,
	) -> Result<(), DispatchError> {
		escrow_transfer(
			gas_meter,
			TransferCause::Call,
			TransactorKind::Contract,
			&self.call_context.ctx.self_account.clone(),
			&to,
			value,
			self.config
		)
	}

	fn terminate(
		&mut self,
		beneficiary: &AccountIdOf<Self::T>,
		gas_meter: &mut GasMeter<Self::T>,
	) -> Result<(), DispatchError> {
		self.call_context.terminate(beneficiary, gas_meter)
	}

	fn call(
		&mut self,
		to: &T::AccountId,
		value: BalanceOf<T>,
		gas_meter: &mut GasMeter<T>,
		input_data: Vec<u8>,
	) -> ExecResult {
		self.call_context.call(to, value, gas_meter, input_data)
	}

	fn restore_to(
		&mut self,
		dest: AccountIdOf<Self::T>,
		code_hash: CodeHash<Self::T>,
		rent_allowance: BalanceOf<Self::T>,
		delta: Vec<StorageKey>,
	) -> Result<(), &'static str> {
		self.call_context.restore_to(dest, code_hash, rent_allowance, delta)
	}

	fn caller(&self) -> &T::AccountId {
		&self.caller
	}

	fn address(&self) -> &T::AccountId {
		&self.call_context.ctx.self_account
	}

	fn balance(&self) -> BalanceOf<T> {
		T::Currency::free_balance(&self.call_context.ctx.self_account)
	}

	fn value_transferred(&self) -> BalanceOf<T> {
		self.value_transferred
	}

	fn now(&self) -> &MomentOf<T> {
		&self.timestamp
	}

	fn minimum_balance(&self) -> BalanceOf<T> {
		self.config.existential_deposit
	}

	fn tombstone_deposit(&self) -> BalanceOf<T> {
		self.config.tombstone_deposit
	}

	fn random(&self, subject: &[u8]) -> SeedOf<T> {
		T::Randomness::random(subject)
	}

	fn deposit_event(&mut self, topics: Vec<T::Hash>, data: Vec<u8>) {
		self.call_context.deposit_event(topics, data);
	}

	fn set_rent_allowance(&mut self, rent_allowance: BalanceOf<T>) {
		self.call_context.set_rent_allowance(rent_allowance);
	}

	fn rent_allowance(&self) -> BalanceOf<T> {
		self.call_context.rent_allowance()
	}

	fn block_number(&self) -> T::BlockNumber { self.block_number }

	fn max_value_size(&self) -> u32 {
		self.config.max_value_size
	}

	fn get_weight_price(&self, weight: Weight) -> BalanceOf<Self::T> {
		self.call_context.get_weight_price(weight)
	}
}
