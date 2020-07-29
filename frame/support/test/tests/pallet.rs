#[frame_support::pallet]
mod pallet {
	type OriginFor<T> = <T as frame_system::Trait>::Origin;

	use frame_support::traits::Get;
	use frame_support::dispatch::DispatchResultWithPostInfo;
	use sp_inherents::ProvideInherent;
	use sp_inherents::InherentData;
	use sp_inherents::InherentIdentifier;
	pub trait ModuleInterface {}
	pub trait Instance {}
	pub struct DefaultInstance;
	impl Instance for DefaultInstance {}

	#[pallet::trait_]
	pub trait Trait<I: Instance = DefaultInstance>: frame_system::Trait {
		#[pallet::const_]
		type Too: Get<u32>;
		type Balance;
	}

	#[pallet::module]
	pub struct Module<T, I = DefaultInstance>(core::marker::PhantomData::<(T, I)>);

	#[pallet::module_interface]
	impl<T: Trait<I>, I: Instance> ModuleInterface for Module<T, I> {
	}

	#[pallet::call]
	impl<T: Trait<I>, I: Instance> Call for Module<T, I> {
		#[pallet::weight = 0]
		fn toto(origin: OriginFor<T>, #[pallet::compact] toto: u32) -> DispatchResultWithPostInfo {
			let _ = origin;
			let _ = toto;
			Ok(().into())
		}
	}

	pallet::storage!(
		pub MyStorage get(fn my_storage) build(|config: &GenesisConfig<T, I>| {
			config.balances.iter().fold(Zero::zero(), |acc: T::Balance, &(_, n)| acc + n)
		}): T::Balance;
	);

	pallet::extra_genesis!(
		fn build(|config| {
		})
	);

	trait StorageMap {
		/// Key type to insert
		type Key: Codec;

		/// Hasher to use
		type Hasher: StoragHasher;

		/// Query type to store
		type Query: Codec;

		/// If not provided, macro will expand to Query::default().
		///
		/// Default value if trie doesn't contains any value.
		fn default() -> Self::Query;
		// NOTE: this was before the syntax `= ..` before

		/// Automatically filled by macro: if Query is option then value is only inner type,
		/// otherwise Query == Value
		///
		/// Value stored on chain.
		type Value: Codec;

		/// Automatically filled by macro (using function default above)
		fn from_optional_value_to_query(v: Option<Self::Value>) -> Self::Query;
		/// Automatically filled by macro.
		fn from_query_to_optional_value(v: Self::Query) -> Option<Self::Value>;

		/// Automatically filled by macro (using the name of the pallet)
		fn module_prefix() -> &'static [u8];

		/// Automatically filled by macro (using the name of the storage)
		fn storage_prefix() -> &'static [u8];

		// All operation on storage are automatically implemented
		// (it is already the case in substrate)

		fn get(..) -> .. { ..}
		fn remove(..) -> .. { ..}
		fn mutate(..) -> .. { ..}
		...
	}


	#[pallet::storage(get(fn my_storage))]
	impl<T: Trait<I>, I: Instance> StorageValue for MyStorage<T, I> {
		type Query = T::Balance;
	}

	#[pallet::storage(get(fn my_storage_2))]
	impl StorageMap for MyStorage2 {
		type Key = T::AccountId; type Hasher = Blake2_256; type Query = Option<T::Balance>;
	}

	#[pallet::storage(get(fn my_storage_3))]
	impl StorageDoubleMap for MyStorage3 {
		type Key1 = T::AccountId; type Hasher1 = Blake2_256;
		type Key2 = T::AccountId; type Hasher2 = Blake2_256;
		type Query = Option<T::Balance>;
	}

	#[pallet::genesis_config_def]
	pub struct GenesisConfig {
		// fields
	}

	#[pallet::genesis_config_build]
	impl GenesisConfig {
		fn build(&self) -> {
		}
	}

	#[pallet::error]
	pub enum Error<T, I = DefaultInstance> {
		/// E
		/// E
		E,
		///
		B,
	}

	#[pallet::event]
	pub enum Event<T: Trait<I>, I: Instance = DefaultInstance> {
		/// A
		A(T::Balance, T::Balance, u32),
		/// B
		/// B2
		B { aa: u32, bb: T::Balance },
	}

	#[pallet::origin]
	pub struct Origin<T, I = DefaultInstance>(core::marker::PhantomData<(T, I)>);

	#[pallet::inherent]
	impl<T: Trait<I>, I: Instance> ProvideInherent for Module<T, I> {
		type Call = Call<T, I>;
		type Error = super::InherentError;

		const INHERENT_IDENTIFIER: InherentIdentifier = super::INHERENT_IDENTIFIER;

		fn create_inherent(_data: &InherentData) -> Option<Self::Call> {
			unimplemented!();
		}
	}
}

#[derive(codec::Encode, sp_runtime::RuntimeDebug)]
#[cfg_attr(feature = "std", derive(codec::Decode))]
pub enum InherentError {
}

impl sp_inherents::IsFatalError for InherentError {
	fn is_fatal_error(&self) -> bool {
		unimplemented!();
	}
}

pub const INHERENT_IDENTIFIER: sp_inherents::InherentIdentifier = *b"testpall";
