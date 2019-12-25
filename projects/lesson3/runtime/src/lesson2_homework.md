作业：

# 1 create 这里面的kitties_count有溢出的可能性，修复 这个问题
```shell
    let new_count = count.checked_add(1).ok_or("Overflow adding a new kitty")?;
    KittiesCount::put(new_count);
```            

# 2 设计加密猫模块 V2
## 链上存储加密猫数据
```shell
decl_storage! {
    trait Store for Module<T: Trait> as KittyShop {
        KittyValue: u32;
    }
}

```

## 每个⽤户可以拥有零到多只猫
```shell

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        fn set_value(origin, value: u32) -> Result {
            let sender = ensure_signed(origin)?;
            KittyValue::put(value);
            Ok(())
        }
    }
}
```

## 遍历所有加密猫
```shell
pub struct KittyStruct<Hash, T::Balance> {
    id: T::Hash,
    dna: T::Hash,
    price: T::Balance,
    gen: u64,
}

decl_storage! {
    trait Store for Module<T: Trait> as KittyShop {
        KittiesMap: map T::AccountId => KittyStruct<T::Balance, T::Hash>;

        //只要给出主人
        KittyOwner get(owner): map T::Hash => Option<T::AccountId>;

    }
}

```

## 每只猫只有⼀个主⼈
```shell
 let kitty_hash: u128 = random_hash();
 ensure!(!<KittyOwner<T>>::exists(kitty_hash), "Kitty already have owner");

```

## 每只猫都有⾃⼰的dna，为 128bit 的数据

```shell
    let nonce = <Nonce<T>>::get();
    let random_hash : u128 = (<system::Module<T>>::random_seed(), &sender, nonce)
            .using_encoded(<T as system::Trait>::Hashing::hash);

```

## 遍历⽤户拥有的所有猫

```shell
decl_storage! {
    trait Store for Module<T: Trait> as KittyShop {
        KittiesList get(dna): map u32 => T::AccountId;
        KittiesCount get(num_of_kittyies): u32;
    }
}

```

## 设计如何⽣成dna (伪代码算法)

```shell
dna 的伪代码算法，除了利用区块index、区块高度、nonce及AccountId增强熵之外，我认为可以利用比特币bip39助记词方式生成dna:
let seed = gen_bip39(24); //默认24，可以12，16等
let random_hash : u128 = (<system::Module<T>>::random_seed(), &sender, seed)
            .using_encoded(<T as system::Trait>::Hashing::hash);
```