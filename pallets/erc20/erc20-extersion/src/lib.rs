#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Encode, Decode};

use frame_support::debug::{self, error, native};
use frame_support::traits::Randomness;

use pallet_contracts::chain_extension::{
	ChainExtension, Environment, Ext, InitState, RetVal, SysConfig, UncheckedFrom,
};

use sp_runtime::{DispatchError, RuntimeDebug};


pub trait Config: pallet_contracts::Config + pallet_erc20::Config {
	type Randomness : Randomness<Self::Hash>;
}

#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug)]
pub struct NewInputParam<Balance> {
	total_supply: Balance,
}

#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug)]
pub struct BalanceOfInputParam<AccountId> {
	owner: AccountId,
}

#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug)]
pub struct AllowanceOfInputParam<AccountId> {
	owner: AccountId,
	spender: AccountId,
}


#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug)]
pub struct TransferInputParam<AccountId, Balance> {
	to: AccountId,
	value: Balance,
}

#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug)]
pub struct AllowanceInputParam<AccountId, Balance> {
	owner: AccountId,
	spender: AccountId,
	value: Balance,
}


#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug)]
pub struct TransferFromInputParam<AccountId, Balance> {
	from: AccountId,
	to: AccountId,
	value: Balance,
}

#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug)]
pub struct TransferHelpInputParam<AccountId, Balance> {
	from: AccountId,
	to: AccountId,
	value: Balance,
}


pub struct FetchRandomExtension;

impl<C: Config> ChainExtension<C> for FetchRandomExtension {
	fn call<E>(func_id: u32, env: Environment<E, InitState>) -> Result<RetVal, DispatchError>
		where
			E: Ext<T = C>,
			<E::T as SysConfig>::AccountId: UncheckedFrom<<E::T as SysConfig>::Hash> + AsRef<[u8]>,
	{

		match func_id {
			1101 => { // function random_seed()
				debug::info!("func_id 1101");
				let mut env = env.buf_in_buf_out();
				// let random_seed: [u8; 32] = super::RandomnessCollectiveFlip::random_seed().0;
				let random_slice = <E::T as Config>::Randomness::random_seed().encode();
				// let random_slice = random_seed.encode();
				native::trace!(
					target: "runtime",
					"[ChainExtension]|call|func_id:{:}",
					func_id
				);
				env.write(&random_slice, false, None)
					.map_err(|_| DispatchError::Other("ChainExtension failed to call random"))?;
			}
			1102 => { // pallet-erc20 function create()
				debug::info!("func_id create() 1102");
				let mut env = env.buf_in_buf_out();
				let caller = env.ext().caller().clone();
				debug::info!("caller: {:?}", caller);

				let input: NewInputParam<
					<E::T as pallet_erc20::Config>::Balance
				> = env.read_as()?;
				debug::info!("input: {:?}", input);

				pallet_erc20::Module::<E::T>::create(caller, input.total_supply)?;
			}
			1103 => { // pallet-erc20 function total_supply
				debug::info!("func_id total_supply() 1103");
				let mut env = env.buf_in_buf_out();
				let caller = env.ext().caller().clone();
				debug::info!("caller: {:?}", caller);

				let balance: u128 = pallet_erc20::Module::<E::T>::total_supply().into();
				debug::info!("balance: {:?}", balance);

				let balance_slice = balance.to_be_bytes();
				debug::info!("balance_silce: {:?}", balance_slice);

				debug::native::trace!(
					target: "runtime",
					"[ChainExtension]|call|func_id:{:}",
					func_id
				);
				env.write(&balance_slice, false, None)
					.map_err(|_| DispatchError::Other("ChainExtension failed to call create collection"))?;
			}
			1104 => { // pallet-erc20 function balance_of
				debug::info!("func_id balance_of 1104");
				let mut env = env.buf_in_buf_out();
				let caller = env.ext().caller().clone();
				debug::info!("caller : {:?}", caller);

				let input: BalanceOfInputParam<
					<E::T as SysConfig>::AccountId
				> = env.read_as()?;
				debug::info!("input: {:?}",input);

				let balance: u128 = pallet_erc20::Module::<E::T>::balance_of(input.owner).into();
				debug::info!("balance: {:?}", balance);

				let balance_slice = balance.to_be_bytes();
				debug::info!("balance_slice: {:?}", balance_slice);

				debug::native::trace!(
					target: "runtime",
					"[ChainExtension]|call|func_id:{:?}",
					func_id
				);

				env.write(&balance_slice, false, None)
					.map_err(|_| DispatchError::Other("ChainExtension failed to call create collection"))?;
			}
			1105 => { // pallet-erc20 function allowance_of
				debug::info!("func_id allowance_of 1105");
				let mut env = env.buf_in_buf_out();
				let caller = env.ext().caller().clone();
				debug::info!("caller : {:?}", caller);

				let input: AllowanceOfInputParam<
					<E::T as SysConfig>::AccountId
				> = env.read_as()?;
				debug::info!("input: {:?}", input);

				let balance: u128 = pallet_erc20::Module::<E::T>::allowance_of(input.owner, input.spender).into();
				debug::info!("balance: {:?}", balance);

				let balance_slice = balance.to_be_bytes();
				debug::info!("balance_slice: {:?}", balance_slice);

				debug::native::trace!(
					target: "runtime",
					"[ChainExtension]|call|func_id:{:?}",
					func_id
				);

				env.write(&balance_slice, false, None)
					.map_err(|_| DispatchError::Other("ChainExtension failed to call create collection"))?;
			}

			1106 => { // pallet-erc20 function transfer_from
				debug::info!("func_id transfer_from 1106");
				let mut env = env.buf_in_buf_out();
				let caller = env.ext().caller().clone();
				debug::info!("caller: {:?}", caller);

				let input: TransferFromInputParam<
					<E::T as SysConfig>::AccountId,
					<E::T as pallet_erc20::Config>::Balance
				> = env.read_as()?;
				debug::info!("input: {:?}", input);

				pallet_erc20::Module::<E::T>::do_transfer_from(input.from, input.to, input.value)?;
			}

			1107 => { // pallet-erc20 function transfer_help
				debug::info!("func_id transfer_help 1107");
				let mut env = env.buf_in_buf_out();
				let caller = env.ext().caller().clone();
				debug::info!("caller: {:?}", caller);

				let input: TransferHelpInputParam<
					<E::T as SysConfig>::AccountId,
					<E::T as pallet_erc20::Config>::Balance
				> = env.read_as()?;
				debug::info!("input: {:?}", input);

				pallet_erc20::Module::<E::T>::transfer_help(input.from, input.to, input.value)?;
			}

			1108 => { // pallet-erc20  function allowance
				debug::info!("func_id allowance");
				let mut env = env.buf_in_buf_out();
				let caller = env.ext().caller().clone();
				debug::info!("caller: {:?}", caller);

				let input: AllowanceInputParam<
					<E::T as SysConfig>::AccountId,
					<E::T as pallet_erc20::Config>::Balance,
				> = env.read_as()?;
				debug::info!("input: {:?}", input);

				pallet_erc20::Module::<E::T>::do_allowance(input.owner, input.spender, input.value)?;
			}

			_ => {
				error!("call an unregistered `func_id`, func_id:{:}", func_id);
				return Err(DispatchError::Other("Unimplemented func_id"));
			}
		}
		Ok(RetVal::Converging(0))
	}

	fn enabled() -> bool {
		true
	}
}