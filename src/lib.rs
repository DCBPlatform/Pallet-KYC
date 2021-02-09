#![cfg_attr(not(feature = "std"), no_std)]


use frame_support::{
	// codec::{Decode, Encode},
	decl_event, decl_module, decl_storage,
	dispatch::DispatchResult,
};
use frame_system::{self as system, ensure_signed, ensure_root};
//use sp_runtime::RuntimeDebug;
use sp_std::prelude::*;


#[cfg(test)]
mod tests;

pub trait Trait: balances::Trait + system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

type AccountIdOf<T> = <T as system::Trait>::AccountId;
// type PersonalInformationOf<T> = PairInfo<AccountIdOf<T>, <T as system::Trait>::BlockNumber>;




decl_storage! {
	trait Store for Module<T: Trait> as KYCStore {
		pub Level get(fn level): map hasher(blake2_128_concat) AccountIdOf<T> => u8;
		pub IsChecker get(fn is_checker): map hasher(blake2_128_concat) AccountIdOf<T> => bool;
		pub Checkers get(fn checkers): Vec<AccountIdOf<T>>;
		pub IsUser get(fn checker): map hasher(blake2_128_concat) AccountIdOf<T> => bool;
		pub Users get(fn users): Vec<AccountIdOf<T>>;
	}
}

decl_event! (
	pub enum Event<T>
	where
	<T as system::Trait>::AccountId,
	{
		/// User added. \[User Account ID\]
		UserAdded(AccountId),
		/// User verified. \[User Account ID, Level of Verification\]
		UserVerified(AccountId, u8),					
		/// Checker added. \[Checker Account ID\]
		CheckerAdded(AccountId),
		/// Checker removed. \[Chcker Account ID\]
		CheckerRemoved(AccountId),
	}
);


decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;


		#[weight = 10_000]
		fn add_checker(origin, new_checker: AccountIdOf<T>) -> DispatchResult {
			let _ = ensure_root(origin);
			<IsChecker<T>>::insert(&new_checker, true);
			
			let mut checkers = Checkers::<T>::get();
		
			match checkers.binary_search(&new_checker) {
				Ok(_) => Ok(()),
				Err(index) => {
					checkers.insert(index, new_checker.clone());
					Checkers::<T>::put(checkers);
					Self::deposit_event(RawEvent::CheckerAdded(new_checker));
					Ok(())
				}
			}			
		}	
		
		#[weight = 10_000]
		fn remove_checker(origin, old_checker: AccountIdOf<T>) -> DispatchResult {
			let _ = ensure_root(origin);
			<IsChecker<T>>::insert(&old_checker, false);		
			
			let mut checkers = Checkers::<T>::get();

			match checkers.binary_search(&old_checker) {

				Ok(index) => {
					checkers.remove(index);
					Checkers::<T>::put(checkers);
					Self::deposit_event(RawEvent::CheckerRemoved(old_checker));
					Ok(())
				},
				Err(_) => Ok(()),
			}			
		}		
		
		#[weight = 10_000]
		fn add_user(origin, user: AccountIdOf<T>) -> DispatchResult {
			let _ = ensure_root(origin);
			<IsUser<T>>::insert(&user, true);
			
			let mut users = Users::<T>::get();
		
			match users.binary_search(&user) {
				Ok(_) => Ok(()),
				Err(index) => {
					users.insert(index, user.clone());
					Users::<T>::put(users);
					Self::deposit_event(RawEvent::UserAdded(user));
					Ok(())
				}
			}				
		}	
		
		#[weight = 10_000]
		fn update_user(origin, user: AccountIdOf<T>, level: u8) -> DispatchResult {
			let caller = ensure_signed(origin)?;

			let checkers = Checkers::<T>::get();

			match checkers.binary_search(&caller) {

				Ok(_index) => {

					let users = Users::<T>::get();

					match users.binary_search(&user) {
		
						Ok(_index) => {	
							Level::<T>::insert(&user, &level);									
							Self::deposit_event(RawEvent::UserVerified(user, level));
							Ok(())
						},
						Err(_) => Ok(()),
					}	
					

				},
				Err(_) => Ok(()),
			}					

			
		}			


	}
}

impl<T: Trait> Module<T> {}