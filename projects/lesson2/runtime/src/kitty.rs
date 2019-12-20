use codec::{Decode, Encode};
use rstd::vec::Vec;
// use rstd::cmp;
use sp_runtime::traits::Hash;
use support::{
    decl_event, decl_module, decl_storage,
    dispatch::Result,
    ensure, print, StorageMap, StorageValue,traits::Randomness,
};
use system::ensure_signed;

pub trait Trait: balances::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

//Token struct
#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct Token<AccountId> {
    name: Vec<u8>,
    symbol: Vec<u8>,
    decimal: u32,
    total: u128,
    owner: AccountId,
}

//Kitty struct
#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct Kitty<Hash, Balance> {
    id: u128,
    dna: Hash,
    price: Balance,
    generation: u64,
    sex: u8,
}

//storage
decl_storage! {
    trait Store for Module<T: Trait> as KittyModule {
        //Kitty
        pub KittyCount get(kitty_count):u128;
        pub Kitties get(kittys):map u128=>Kitty<T::Hash,T::Balance>;
        pub KittyOwner get(kitty_owner):map u128=>T::AccountId;
        pub KittyHashIndex get(kitty_hash_index): map T::Hash => u128;
        pub OwnerKittyCount get(owner_kitty_count):map T::AccountId=>u128;
        pub OwnerKittyList get(owner_kitty_list):map (T::AccountId,u128)=>u128;
        pub OwnerKittyIndex get(owner_kitty_index):map u128=>u128;
        //Token
        pub Tokens get(token):map Vec<u8> => Token<T::AccountId>;
        pub FreeBalance get(free_balance):map T::AccountId=>T::Balance;
    }
}
//extrinsics call
decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;
        //创建猫咪
        pub fn create_kitty(origin,value:u64)->Result{
            let account = ensure_signed(origin)?;
            let dna = Self::kitty_dna(account);
            print("create dna");
            Ok(())
        }
        // 繁殖小猫
         pub fn breed(origin,kitty_first:u128,kitty_second:u128)->Result{
             Ok(())
         }
         //挂单出售
         pub fn sell_kitty(origin,kitty_id:u128)->Result{
             Ok(())
         }
         //出价购买
         pub fn buy_kitty(origin,kitty_id:u128,balance:T::Balance)->Result{
             Ok(())
         }
         //赠送
         pub fn present(origin,kitty_id:u128,to_account:T::AccountId)->Result{
             Ok(())
         }
         //Token
         //转账
         pub fn transfer(origin,to:T::AccountId,token:Vec<u8>,balance:T::Balance, memo:Vec<u8>)->Result{
             Ok(())
         }
         //创建erc20Token
         pub fn create_token(origin,name:Vec<u8>,symbol:Vec<u8>,decimal:Vec<u8>,total:Vec<u8>)->Result{
             Ok(())
         }
    }
}

impl<T: Trait> Module<T> {
    fn kitty_dna(owner: T::AccountId) -> T::Hash{
        let random_hash = <randomness_collective_flip::Module<T>>::random_seed();
        random_hash
    }
}
decl_event! {
    pub enum Event<T> where <T as balances::Trait>::Balance,<T as system::Trait>::AccountId,<T as system::Trait>::Hash{
        Create(AccountId,Hash,u128),
        Sell(AccountId,Hash,u128,Balance),
        Transfered(AccountId,AccountId,Hash),
        Buy(AccountId,AccountId,Hash,u128,Balance),
    }
}

//SKIP_WASM_BUILD= cargo check