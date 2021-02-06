#![cfg_attr(not(feature = "std"), no_std)]


use frame_support::{
	codec::{Decode, Encode},
	decl_event, decl_module, decl_storage,
	dispatch::DispatchResult,
};
use frame_system::{self as system, ensure_signed};
use sp_runtime::RuntimeDebug;
use sp_std::prelude::*;


#[cfg(test)]
mod tests;

pub trait Trait: balances::Trait + system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug)]
pub struct PersonalInformation {
	name: Vec<u8>,
	phone: Vec<u8>,
	email: Vec<u8>,
	address: Vec<u8>,
	income: Vec<u8>,
}


decl_storage! {
	trait Store for Module<T: Trait> as KYCStore {
		KYCLevel get(fn kyc_level): map hasher(blake2_128_concat) AccountId => u8;
		Checker get(fn checker): map hasher(blake2_128_concat) AccountId => bool;
	}
}

decl_event! (
	pub enum Event<T>
	where
		<T as system::Trait>::AccountId,
		<T as system::Trait>::Hash,
		<T as balances::Trait>::Balance
	{
		CheckerAdded(AccountId),
		CheckerRemoved(AccountId)
	}
);

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;


		#[weight = 10_000]
		fn update_information(origin, kata: Vec<u8>, number: u32, hash: T::Hash, balance: T::Balance) -> DispatchResult {
			let _ = ensure_signed(origin)?;
			let thing = InnerThing {
							kata,
							number,
							hash,
							balance,
						};
			<InnerThingsByNumbers<T>>::insert(number, thing);
			Self::deposit_event(RawEvent::NewInnerThing(number, hash, balance));
			Ok(())
		}	
		
		#[weight = 10_000]
		fn view_information(origin, kata: Vec<u8>, number: u32, hash: T::Hash, balance: T::Balance) -> DispatchResult {
			let _ = ensure_signed(origin)?;
			let thing = InnerThing {
							kata,
							number,
							hash,
							balance,
						};
			<InnerThingsByNumbers<T>>::insert(number, thing);
			Self::deposit_event(RawEvent::NewInnerThing(number, hash, balance));
			Ok(())
		}	
		
		#[weight = 10_000]
		fn add_checker(origin, checker: AccountId) -> DispatchResult {
			let _ = ensure_root(origin)?;
			<Checker>::put(checker, true)
			Ok(())
		}	
		
		#[weight = 10_000]
		fn remove_checker(origin, checker: AccountId) -> DispatchResult {
			let _ = ensure_root(origin)?;
			<Checker>::put(checker, false)
			Ok(())
		}			


	}
}

impl<T: Trait> Module<T> {

	pub fn get_information(who: AccountId, what: Vec<u8>) -> Vec<u8> {
	}

	pub fn set_information(who: AccountId, what: Vec<u8>) -> () {
	}
}