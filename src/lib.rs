#![cfg_attr(not(feature = "std"), no_std)]


use frame_support::{
	codec::{Decode, Encode},
	decl_event, decl_module, decl_storage,
	dispatch::DispatchResult,
};
use frame_system::{self as system, ensure_signed, ensure_root};
use sp_runtime::RuntimeDebug;
use sp_std::prelude::*;


#[cfg(test)]
mod tests;

pub trait Trait: balances::Trait + system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

type AccountIdOf<T> = <T as system::Trait>::AccountId;
// type PersonalInformationOf<T> = PairInfo<AccountIdOf<T>, <T as system::Trait>::BlockNumber>;


#[derive(Encode, Decode, Clone, Default, RuntimeDebug)]
pub struct PersonalInformation {
	name: Vec<u8>,
	phone: Vec<u8>,
	email: Vec<u8>,
	address: Vec<u8>,
	income: Vec<u8>,
	nric: Vec<u8>,
	passport: Vec<u8>,
	verified: bool
}


decl_storage! {
	trait Store for Module<T: Trait> as KYCStore {
		pub Level get(fn level): map hasher(blake2_128_concat) AccountIdOf<T> => u8;
		pub Checker get(fn checker): map hasher(blake2_128_concat) AccountIdOf<T> => bool;
		pub User get(fn user): map hasher(blake2_128_concat) AccountIdOf<T> => Option<PersonalInformation>;
		pub Users get(fn users): Vec<AccountIdOf<T>>;
	}
}

decl_event! (
	pub enum Event<T>
	where
	<T as system::Trait>::AccountId,
	{
		/// Checker added. \[accountId\]
		CheckerAdded(AccountId),
		/// Checker removed. \[accountId\]
		CheckerRemoved(AccountId),
	}
);

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;


		#[weight = 10_000]
		fn update_information(origin, 
			name:Vec<u8>, 
			phone:Vec<u8>, 
			email:Vec<u8>, 
			address:Vec<u8>, 
			income:Vec<u8>, 
			nric:Vec<u8>, 
			passport:Vec<u8>) -> DispatchResult {
			let caller = ensure_signed(origin)?;
			let salah: bool = false;
			let thing = PersonalInformation {
				name: name, 
				phone: phone, 
				email: email, 
				address: address, 
				income: income, 
				nric: nric, 
				passport: passport, 
				verified: salah
			};
			<User<T>>::insert(&caller, thing);
			Ok(())
		}		

		#[weight = 10_000]
		fn validate_information(origin, account: AccountIdOf<T>) -> DispatchResult {
			let caller = ensure_signed(origin)?;
			let existing_information = <User<T>>::get(&account);
			
			let benar: bool = true;
			let thing = PersonalInformation {
				name: existing_information.clone().unwrap().name,
				phone: existing_information.clone().unwrap().phone,
				email: existing_information.clone().unwrap().email,
				address: existing_information.clone().unwrap().address,
				income: existing_information.clone().unwrap().income,
				nric: existing_information.clone().unwrap().nric,
				passport: existing_information.clone().unwrap().passport,
				verified: benar
			};
			<User<T>>::insert(&account, thing);
			let mut users = <Users<T>>::get();
			let index = users.len();
			users.insert(index, account);
			<Users<T>>::put(users);

			Ok(())
		}			
	
		
		#[weight = 10_000]
		fn add_checker(origin, checker: AccountIdOf<T>) {
			let _ = ensure_root(origin);
			<Checker<T>>::insert(checker, true)					
		}	
		
		#[weight = 10_000]
		fn remove_checker(origin, checker: AccountIdOf<T>) {
			let _ = ensure_root(origin);
			<Checker<T>>::insert(checker, false);					
		}			


	}
}

impl<T: Trait> Module<T> {

	// pub fn get_information(who: AccountIdOf<T>, what: Vec<u8>) -> () {
	// }

	// pub fn set_information(who: AccountIdOf<T>, what: Vec<u8>) -> () {
	// }
}