#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Encode, Decode};

use frame_support::debug;
use frame_support::traits::Randomness;
use pallet_contracts::chain_extension::{
	ChainExtension, Environment, Ext, InitState, RetVal, SysConfig, UncheckedFrom,
};
use sp_runtime::{DispatchError, RuntimeDebug};
use sp_std::prelude::*;

pub trait Config: pallet_contracts::Config + pallet_erc1155::Config {
	type Randomness: Randomness<Self::Hash>;
}

/// Result that returns a [`DispatchError`] on error.
pub type Result<T> = sp_std::result::Result<T, DispatchError>;

// function id 1001 do_set_approval_for_all(owner: T::AccountId, spender: T::AccountId, approved: bool);
#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug)]
pub struct SetApprovalForAllInputParam<AccountId> {
	owner: AccountId,
	operator: AccountId,
	approved: bool,
}
// function id 1002 do_mint(to: T::AccountId, id: T::TokenId, amount: T::TokenBalance)
#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug)]
pub struct MintInputParam<AccountId, TokenId, TokenBalance> {
	to: AccountId,
	id: TokenId ,
	amount: TokenBalance,
}

// function id 1003 do_batch_mint(to: T::AccountId,  ids: Vec<T::TokenId>, amounts: Vec<T::TokenBalance>)
#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug)]
pub struct BatchMintParam<AccountId, TokenId, TokenBalance> {
	to: AccountId,
	ids: Vec<TokenId>,
	amounts: Vec<TokenBalance>,
}

// function id 1004 do_burn(from: T::AccountId, to: T::TokenId, amount: T::TokenBalance)
#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug)]
pub struct BurnInputParam<AccountId, TokenId, TokenBalance> {
	from: AccountId,
	to: TokenId ,
	amount: TokenBalance,
}

// function id 1005 do_batch_burn(from: T::AccountId, ids: Vec<T::TokenId>, amount: Vec<T::TokenBalance>)
#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug)]
pub struct BatchBurnInputParam<AccountId, TokenId, TokenBalance> {
	from: AccountId,
	ids: Vec<TokenId>,
	amounts: Vec<TokenBalance>,
}
// function id 1006 do_transfer_from(from: T::AccountId, to: T::AccountId, id: T::TokenId, amount: T::TokenBalance)
#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug)]
pub struct TransferFromInputParam<AccountId, TokenId, TokenBalance> {
	from: AccountId,
	to: AccountId,
	id: TokenId ,
	amount: TokenBalance,
}
// function id 1007 do_batch_transfer_from(from: T::AccountId, to: T::AccountId, ids: Vec<T::TokenId>, amounts: Vec<T::TokenBalance>)
#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug)]
pub struct BatchTransferInputParam<AccountId, TokenId, TokenBalance> {
	from: AccountId,
	to: AccountId,
	ids: Vec<TokenId>,
	amounts: Vec<TokenBalance>,
}

// function id 1008 approved_or_owner(who: T::AccountId, account: T::AccountId)
#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug)]
pub struct ApprovedOrOwnerInputParam<AccountId> {
	who: AccountId,
	account: AccountId,
}

// function id 1009 is_nf(id: T::TokenId)
#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug)]
pub struct IsNfInputParam<TokenId> {
	id: TokenId ,
}
// function id 1010 is_approved_for_all(owner: T::AccountId, operator: T::AccountId)
#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug)]
pub struct IsApprovedForAllInputParam<AccountId> {
	owner: AccountId,
	operator: AccountId
}

// function id 1011 balance_of(owner: T::AccountId, id: T::TokenId)
#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug)]
pub struct BalanceOfInputParam<AccountId, TokenId> {
	owner: AccountId,
	id: TokenId ,
}

// function id 1012 balance_of_batch(owners: Vec<T::AccountId>, ids: Vec<T::TokenId>)
#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug)]
pub struct BalanceOfBatchInputParam<AccountId, TokenId> {
	owners: Vec<AccountId>,
	ids: Vec<TokenId> ,
}

// function id 1013



/// chain extension of contract
pub struct SgcChainExtension;

impl<C: Config> ChainExtension<C> for SgcChainExtension {
	fn call<E>(func_id: u32, env: Environment<E, InitState>) -> Result<RetVal>
	where
		E: Ext<T = C>,
		<E::T as SysConfig>::AccountId: UncheckedFrom<<E::T as SysConfig>::Hash> + AsRef<[u8]>,
	{
		match func_id {
			1000 => {
				debug::info!("run 1001");
				let mut env = env.buf_in_buf_out();
				let random_slice = <E::T as Config>::Randomness::random_seed().encode();
				// let random_slice = random_seed.encode();
				debug::native::trace!(
					target: "runtime",
					"[ChainExtension]|call|func_id:{:}",
					func_id
				);
				env.write(&random_slice, false, None)
					.map_err(|_| DispatchError::Other("ChainExtension failed to call random"))?;
			}
			1001 => { // erc1155 do_set_approval_for_all(owner: &T::AccountId, spender: &T::AccountId, approved: bool);
				debug::info!("run 1001");
				let mut env = env.buf_in_buf_out();
				let caller = env.ext().caller().clone();
				debug::info!("caller: {:?}", caller);

				let in_len = env.in_len();
				debug::info!("in_len: {}", in_len);

				let mut buffer = vec![0u8; in_len as usize];

				env.read_into(&mut &mut buffer[..])?;
				debug::info!("buffer: {:?}", buffer);

				let input: SetApprovalForAllInputParam<
					<E::T as SysConfig>::AccountId
				> = env.read_as()?;
				debug::info!("input: {:?}", input);

				let weight = 100_000;
				env.charge_weight(weight)?;

				pallet_erc1155::Module::<E::T>::do_set_approval_for_all(&input.owner, &input.operator, input.approved)?;
			}
			1002 => { // erc1155 do_mint(to: &T::AccountId, id: &T::TokenId, amount: T::TokenBalance)
				debug::info!("run 1002");
				let mut env = env.buf_in_buf_out();
				let caller = env.ext().caller().clone();
				debug::info!("caller: {:?}", caller);

				let in_len = env.in_len();
				debug::info!("in_len: {}", in_len);

				let mut buffer = vec![0u8; in_len as usize];

				env.read_into(&mut &mut buffer[..])?;
				debug::info!("buffer: {:?}", buffer);

				let input: MintInputParam<
					<E::T as SysConfig>::AccountId,
					<E::T as pallet_erc1155::Config>::TokenId,
					<E::T as pallet_erc1155::Config>::TokenBalance,
				> = env.read_as()?;
				debug::info!("input: {:?}", input);

				let weight = 100_000;
				env.charge_weight(weight)?;

				pallet_erc1155::Module::<E::T>::do_mint(&input.to, &input.id, input.amount)?;

			}
			1003 => { //erc1155 do_batch_mint(to: &T::AccountId,  ids: &Vec<T::TokenId>, amounts: Vec<T::TokenBalance>)
				debug::info!("run 1003");
				let mut env = env.buf_in_buf_out();
				let caller = env.ext().caller().clone();
				debug::info!("caller: {:?}", caller);

				let in_len = env.in_len();
				debug::info!("in_len: {}", in_len);

				let mut buffer = vec![0u8; in_len as usize];

				env.read_into(&mut &mut buffer[..])?;
				debug::info!("buffer: {:?}", buffer);

				let input: BatchMintParam<
					<E::T as SysConfig>::AccountId,
					<E::T as pallet_erc1155::Config>::TokenId,
					<E::T as pallet_erc1155::Config>::TokenBalance,
				> = env.read_as()?;
				debug::info!("input: {:?}", input);

				let weight = 100_000;
				env.charge_weight(weight)?;

				pallet_erc1155::Module::<E::T>::do_batch_mint(&input.to, &input.ids, input.amounts)?;

			}
			1004 => { // erc1155 do_burn(from: &T::AccountId, to: &T::TokenId, amount: T::TokenBalance)
				debug::info!("run 1004");
				let mut env = env.buf_in_buf_out();
				let caller = env.ext().caller().clone();
				debug::info!("caller: {:?}", caller);

				let in_len = env.in_len();
				debug::info!("in_len: {}", in_len);

				let mut buffer = vec![0u8; in_len as usize];

				env.read_into(&mut &mut buffer[..])?;
				debug::info!("buffer: {:?}", buffer);

				let input: BurnInputParam<
					<E::T as SysConfig>::AccountId,
					<E::T as pallet_erc1155::Config>::TokenId,
					<E::T as pallet_erc1155::Config>::TokenBalance,
				> = env.read_as()?;
				debug::info!("input: {:?}", input);

				let weight = 100_000;
				env.charge_weight(weight)?;

				pallet_erc1155::Module::<E::T>::do_burn(&input.from, &input.to, input.amount)?;
			}
			1005 => { //erc1155 do_batch_burn(from: &T::AccountId, ids: &Vec<T::TokenId>, amounts: Vec<T::TokenBalance>)
				debug::info!("run 1005");
				let mut env = env.buf_in_buf_out();
				let caller = env.ext().caller().clone();
				debug::info!("caller: {:?}", caller);

				let in_len = env.in_len();
				debug::info!("in_len: {}", in_len);

				let mut buffer = vec![0u8; in_len as usize];

				env.read_into(&mut &mut buffer[..])?;
				debug::info!("buffer: {:?}", buffer);

				let input: BatchBurnInputParam<
					<E::T as SysConfig>::AccountId,
					<E::T as pallet_erc1155::Config>::TokenId,
					<E::T as pallet_erc1155::Config>::TokenBalance,
				> = env.read_as()?;
				debug::info!("input: {:?}", input);

				let weight = 100_000;
				env.charge_weight(weight)?;

				pallet_erc1155::Module::<E::T>::do_batch_burn(&input.from, &input.ids, input.amounts)?;
			}
			1006 => { // erc1155 do_transfer_from(from: &T::AccountId, to: &T::AccountId, id: &T::TokenId, amount: T::TokenBalance)
				debug::info!("run 1006");
				let mut env = env.buf_in_buf_out();
				let caller = env.ext().caller().clone();
				debug::info!("caller: {:?}", caller);

				let in_len = env.in_len();
				debug::info!("in_len: {}", in_len);

				let mut buffer = vec![0u8; in_len as usize];

				env.read_into(&mut &mut buffer[..])?;
				debug::info!("buffer: {:?}", buffer);

				let input: TransferFromInputParam<
					<E::T as SysConfig>::AccountId,
					<E::T as pallet_erc1155::Config>::TokenId,
					<E::T as pallet_erc1155::Config>::TokenBalance,
				> = env.read_as()?;
				debug::info!("input: {:?}", input);

				let weight = 100_000;
				env.charge_weight(weight)?;

				pallet_erc1155::Module::<E::T>::do_transfer_from(&input.from, &input.to, &input.id, input.amount)?;

			}
			1007 => { // erc1155 do_batch_transfer_from(from: &T::AccountId, to: &T::AccountId, ids: &Vec<T::TokenId>, amounts: Vec<T::TokenBalance>)
				debug::info!("run 1007");
				let mut env = env.buf_in_buf_out();
				let caller = env.ext().caller().clone();
				debug::info!("caller: {:?}", caller);

				let in_len = env.in_len();
				debug::info!("in_len: {}", in_len);

				let mut buffer = vec![0u8; in_len as usize];

				env.read_into(&mut &mut buffer[..])?;
				debug::info!("buffer: {:?}", buffer);

				let input: BatchTransferInputParam<
					<E::T as SysConfig>::AccountId,
					<E::T as pallet_erc1155::Config>::TokenId,
					<E::T as pallet_erc1155::Config>::TokenBalance,
				> = env.read_as()?;
				debug::info!("input: {:?}", input);

				let weight = 100_000;
				env.charge_weight(weight)?;

				pallet_erc1155::Module::<E::T>::do_batch_transfer_from(&input.from, &input.to, &input.ids, input.amounts)?;

			}
			1008 => { // erc1155 approved_or_owner(who: &T::AccountId, account: &T::AccountId)
				debug::info!("run 1006");
				let mut env = env.buf_in_buf_out();
				let caller = env.ext().caller().clone();
				debug::info!("caller: {:?}", caller);

				let in_len = env.in_len();
				debug::info!("in_len: {}", in_len);

				let mut buffer = vec![0u8; in_len as usize];

				env.read_into(&mut &mut buffer[..])?;
				debug::info!("buffer: {:?}", buffer);

				let input: ApprovedOrOwnerInputParam<
					<E::T as SysConfig>::AccountId
				> = env.read_as()?;
				debug::info!("input: {:?}", input);

				let weight = 100_000;
				env.charge_weight(weight)?;

				let ret : bool = pallet_erc1155::Module::<E::T>::approved_or_owner(&input.who, &input.account);
				debug::info!("ret = {:?}", ret);
				let ret = ret as u8;

				let ret_slice = ret.to_be_bytes();
				debug::info!("balance_slice: {:?}", ret_slice);

				debug::native::trace!(
					target: "runtime",
					"[ChainExtension]|call|func_id:{:}",
					func_id
				);

				env.write(&ret_slice, false, None)
					.map_err(|_| DispatchError::Other("ChainExtension failed to call create collection"))?;

			}
			1009 => { // erc1155 is_nf(id: &T::TokenId)
				debug::info!("run 1006");
				let mut env = env.buf_in_buf_out();
				let caller = env.ext().caller().clone();
				debug::info!("caller: {:?}", caller);

				let in_len = env.in_len();
				debug::info!("in_len: {}", in_len);

				let mut buffer = vec![0u8; in_len as usize];

				env.read_into(&mut &mut buffer[..])?;
				debug::info!("buffer: {:?}", buffer);

				let input: IsNfInputParam<
					<E::T as pallet_erc1155::Config>::TokenId
				> = env.read_as()?;
				debug::info!("input: {:?}", input);

				let weight = 100_000;
				env.charge_weight(weight)?;

				let ret : bool = pallet_erc1155::Module::<E::T>::is_nf(&input.id)?;
				debug::info!("ret = {:?}", ret);
				let ret = ret as u8;

				let ret_slice = ret.to_be_bytes();
				debug::info!("balance_slice: {:?}", ret_slice);

				debug::native::trace!(
					target: "runtime",
					"[ChainExtension]|call|func_id:{:}",
					func_id
				);

				env.write(&ret_slice, false, None)
					.map_err(|_| DispatchError::Other("ChainExtension failed to call create collection"))?;

			}
			1010 => { // erc1155 is_approved_for_all(owner: &T::AccountId, operator: &T::AccountId)
				debug::info!("run 1006");
				let mut env = env.buf_in_buf_out();
				let caller = env.ext().caller().clone();
				debug::info!("caller: {:?}", caller);

				let in_len = env.in_len();
				debug::info!("in_len: {}", in_len);

				let mut buffer = vec![0u8; in_len as usize];

				env.read_into(&mut &mut buffer[..])?;
				debug::info!("buffer: {:?}", buffer);

				let input: IsApprovedForAllInputParam<
					<E::T as SysConfig>::AccountId
				> = env.read_as()?;
				debug::info!("input: {:?}", input);

				let weight = 100_000;
				env.charge_weight(weight)?;

				let ret : bool = pallet_erc1155::Module::<E::T>::is_approved_for_all(&input.owner, &input.operator);
				debug::info!("ret = {:?}", ret);
				let ret = ret as u8;

				let ret_slice = ret.to_be_bytes();
				debug::info!("balance_slice: {:?}", ret_slice);

				debug::native::trace!(
					target: "runtime",
					"[ChainExtension]|call|func_id:{:}",
					func_id
				);

				env.write(&ret_slice, false, None)
					.map_err(|_| DispatchError::Other("ChainExtension failed to call create collection"))?;

			}
			1011 => { // erc1155 balance_of(owner: &T::AccountId, id: &T::TokenId)
				debug::info!("run 1011");
				let mut env = env.buf_in_buf_out();
				let caller = env.ext().caller().clone();
				debug::info!("caller: {:?}", caller);

				let input: BalanceOfInputParam<
					<E::T as SysConfig>::AccountId,
					<E::T as pallet_erc1155::Config>::TokenId,
				> = env.read_as()?;

				let balance: u128 = pallet_erc1155::Module::<E::T>::balance_of(&input.owner, &input.id).into();
				debug::info!("balance: {:?}", balance);

				let balance_slice = balance.to_be_bytes();
				debug::info!("balance_slice: {:?}", balance_slice);

				debug::native::trace!(
					target: "runtime",
					"[ChainExtension]|call|func_id:{:}",
					func_id
				);

				env.write(&balance_slice, false, None)
					.map_err(|_| DispatchError::Other("ChainExtension failed to call create collection"))?;
			}
			// 1012 => { // erc1155 balance_of_batch(owners: &Vec<T::AccountId>, ids: &Vec<T::TokenId>)
			// 	debug::info!("run 1012");
			// 	let mut env = env.buf_in_buf_out();
			// 	let caller = env.ext().caller().clone();
			// 	debug::info!("caller: {:?}", caller);
			//
			// 	let input: BalanceOfBatchInputParam<
			// 		<E::T as SysConfig>::AccountId,
			// 		<E::T as pallet_erc1155::Config>::TokenId,
			// 	> = env.read_as()?;
			//
			// 	let batch_balance: Vec<<E::T as pallet_erc1155::Config>::TokenBalance> = pallet_erc1155::Module::<E::T>::balance_of_batch(&input.owners, &input.ids)?;
			// 	debug::info!("balance: {:?}", batch_balance);
			//
			//
			// 	let mut vec_batch_balance = Vec::new();
			// 	for val in batch_balance.iter() {
			// 		let val : u128 = val.into();
			// 		vec_batch_balance.push(val.to_be_bytes());
			// 	}
			//
			// 	// let balance_slice = balance.to_be_bytes();
			// 	// debug::info!("balance_slice: {:?}", balance_slice);
			//
			// 	debug::native::trace!(
			// 		target: "runtime",
			// 		"[ChainExtension]|call|func_id:{:}",
			// 		func_id
			// 	);
			//
			// 	env.write(&vec_batch_balance, false, None)
			// 		.map_err(|_| DispatchError::Other("ChainExtension failed to call create collection"))?;
			// }
			// 1013 => { // erc1155 create()
			//
			// }

			_ => {
				debug::error!("call an unregistered `func_id`, func_id:{:}", func_id);
				return Err(DispatchError::Other("Unimplemented func_id"));
			}
		}
		Ok(RetVal::Converging(0))
	}

	fn enabled() -> bool {
		true
	}
}
