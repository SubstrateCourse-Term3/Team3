use support::{decl_module, decl_storage, StorageValue, StorageMap, traits::Randomness};
use codec::{Encode, Decode};
use runtime_io::hashing::blake2_128;
use system::ensure_signed;

pub trait Trait: system::Trait {
}

//数据结构
#[derive(Encode, Decode, Default)]
pub struct Kitty(pub [u8; 16]);

decl_storage! {
	trait Store for Module<T: Trait> as Kitties {
		/// Stores all the kitties, key is the kitty id / index
		pub Kitties get(fn kitties): map u32 => Kitty;
		/// Stores the total number of kitties. i.e. the next kitty index
		pub KittiesCount get(fn kitties_count): u32;
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		/// Create a new kitty
		pub fn create(origin) {
			let sender = ensure_signed(origin)?;
			let payload = (
				<randomness_collective_flip::Module<T> as Randomness<T::Hash>>::random_seed(),
				sender,
				<system::Module<T>>::extrinsic_index(), <system::Module<T>>::block_number()
			);
			let dna = payload.using_encoded(blake2_128);
			let kitty = Kitty(dna);
			let count = Self::kitties_count();
			Kitties::insert(count, kitty);
			// KittiesCount::put(count + 1);
			//作业: 检查count是否溢出
			let new_count = count.checked_add(1).ok_or("Overflow adding a new kitty")?;
			KittiesCount::put(new_count);
			


		}
	}
}
