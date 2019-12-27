use support::{decl_module, decl_storage,decl_event, StorageValue, StorageMap, traits::Randomness,ensure};
use codec::{Encode, Decode};
use runtime_io::hashing::blake2_128;
use rstd::vec::Vec;
use system::ensure_signed;

pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

#[derive(Encode, Decode, Default,Clone)]
pub struct Kitty(pub [u8; 16]);

decl_storage! {
	trait Store for Module<T: Trait> as Kitties {
		/// Stores all the kitties, key is the kitty id / index
		pub Kitties get(fn kitties): map u32 => Kitty;
		/// Stores the total number of kitties. i.e. the next kitty index
		pub KittyOwner get(fn kitty_owner): map u32 => T::AccountId;

		pub KittiesDna get(fn kitties_dna):map Vec<u8> => u32;
		/// 
		pub KittiesCount get(fn kitties_count): u32;
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;
		/// Create a new kitty
		pub fn create(origin) {
			let sender = ensure_signed(origin)?;
			let payload = (
				<randomness_collective_flip::Module<T> as Randomness<T::Hash>>::random_seed(),
				&sender,
				<system::Module<T>>::extrinsic_index(), <system::Module<T>>::block_number()
			);
			let dna = payload.using_encoded(blake2_128);
			let kitty = Kitty(dna);
			let count = Self::kitties_count();
			let kitty_id = count.checked_add(1).ok_or("overflow when create new kitty")?;
			Self::insert_kitty(&sender,kitty_id,kitty);
			Self::deposit_event(RawEvent::Created(sender,kitty_id));
		}

		pub fn breed_kitty(origin,kitty_id_1:u32,kitty_id_2:u32){
			let sender = ensure_signed(origin)?;
			ensure!(kitty_id_1!=kitty_id_2,"You need input two different kitties");
			ensure!(Kitties::exists(kitty_id_1),"The first kitty does not exist");
			ensure!(Kitties::exists(kitty_id_2),"The second kitty does not exist");
			let kitty_1 = Self::kitties(kitty_id_1);
			let owner_1 = Self::kitty_owner(kitty_id_1);
			ensure!(owner_1 == sender,"You are not the first kitty's owner");
			let kitty_2 = Self::kitties(kitty_id_2);
			let owner_2 = Self::kitty_owner(kitty_id_2);
			ensure!(owner_2 == sender,"You are not the second kitty's owner");
			let payload=(
				<randomness_collective_flip::Module<T> as Randomness<T::Hash>>::random_seed(),
				&sender,
				<system::Module<T>>::extrinsic_index(), <system::Module<T>>::block_number()
			);
			let kitty_1_dna = kitty_1.0;
			let kitty_2_dna = kitty_2.0;
			let random = payload.using_encoded(blake2_128);
			let mut child_dna = [0u8;16];
			for i in 0..child_dna.len(){
				child_dna[i] = if random[i]%2 == 0 {kitty_1_dna[i]} else {kitty_2_dna[i]};
			}
			ensure!(!KittiesDna::exists(child_dna.to_vec()),"DNA is same");
			let new_kitty = Kitty(child_dna);
			let count = Self::kitties_count();
			let kitty_id = count.checked_add(1).ok_or("overflow when create new kitty")?;
			Self::insert_kitty(&sender,kitty_id,new_kitty);
			Self::deposit_event(RawEvent::Created(sender,kitty_id));
		}
	}
}

decl_event!{
	pub enum Event<T> where <T as system::Trait>::AccountId,{
		Created(AccountId,u32),
	}
}

impl<T: Trait> Module<T>{
	fn insert_kitty(owner: &T::AccountId,kitty_id:u32,kitty:Kitty){
		Kitties::insert(kitty_id, kitty.clone());
		KittiesCount::put(kitty_id);
		<KittyOwner<T>>::insert(kitty_id,owner.clone());
		KittiesDna::insert(kitty.0.to_vec(),kitty_id);
	}
}