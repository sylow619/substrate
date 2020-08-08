// This file is part of Substrate.

// Copyright (C) 2020 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::pallet::Def;
use proc_macro2::Span;

/// * generate StoragePrefix structs (e.g. for a storage `MyStorage` a struct with the name
///   `MyStorageP` is generated and implements StorageInstance trait.
/// * TODO TODO: generate metadatas
/// * TODO TODO: maybe assert that storages are correclty written, i.e. they implement their respective
/// trait correctly
pub fn expand_storages(def: &mut Def) -> proc_macro2::TokenStream {
	let scrate = &def.scrate();
	let type_impl_static_gen = &def.type_impl_static_generics();
	let type_use_gen = &def.type_use_generics();
	let module_ident = &def.module.module;

	let prefix_struct_vis = def.storages.iter()
		.map(|storage_def| storage_def.vis.clone());

	let prefix_struct_ident = def.storages.iter()
		.map(|storage_def|
			syn::Ident::new(&format!("{}P", storage_def.ident), storage_def.ident.span())
		);
	let prefix_struct_const = def.storages.iter()
		.map(|storage_def| storage_def.ident.to_string());

	let instance = if def.trait_.has_instance {
		// If trait_ has instance parsing ensure storage is generic over `I`
		syn::Ident::new("I", Span::call_site())
	} else {
		// Otherwise we use __InherentHiddenInstance
		syn::Ident::new(crate::INHERENT_INSTANCE_NAME, Span::call_site())
	};

	let (prefix_struct_impl_gen, prefix_struct_use_gen) = if def.trait_.has_instance {
		(quote::quote!(I: #scrate::traits::Instance), quote::quote!(I))
	} else {
		(Default::default(), Default::default())
	};

	// TODO TODO: everywhere check debug print to format and whitespace stuff for metadata
	let entries = def.storages.iter()
		.map(|storage| {
			let docs = &storage.docs;
			use crate::pallet::parse::storage::Metadata;
			let args = match &storage.metadata {
				Metadata::Value { value } => quote::quote!(stringify!(#value), &[ #( #docs, )* ]), // TODO TODO strinify remove whitespace ?
				Metadata::Map { key, value } => quote::quote!(
					stringify!(#key), stringify!(#value), &[ #( #docs, )* ]
				),
				Metadata::DoubleMap { key1, key2, value } => quote::quote!(
					stringify!(#key1), stringify!(#key2), stringify!(#value), &[ #( #docs, )* ]
				),
			};
			let ident = &storage.ident;
			let instance_gen = if storage.has_instance  {
				Some(quote::quote!(I))
			} else {
				None
			};
			let trait_gen= if storage.has_trait {
				Some(quote::quote!(T,))
			} else {
				None
			};

			quote::quote!(
				<#ident<#trait_gen #instance_gen>>::storage_entry_metadata_builder(#args)
			)
		});

	quote::quote!(
		#(
			#prefix_struct_vis struct #prefix_struct_ident<#prefix_struct_use_gen>(
				core::marker::PhantomData<#prefix_struct_use_gen>
			);
			impl<#prefix_struct_impl_gen> #scrate::traits::StorageInstance
			for #prefix_struct_ident<#prefix_struct_use_gen>
			{
				type I = #instance;
				const STORAGE_PREFIX: &'static str = #prefix_struct_const;
			}
		)*

		impl<#type_impl_static_gen> #module_ident<#type_use_gen> {
			#[doc(hidden)]
			pub fn storage_metadata() -> #scrate::metadata::StorageMetadata {
				#scrate::metadata::StorageMetadata {
					prefix: #scrate::metadata::DecodeDifferent::Encode(#instance::PREFIX),
					entries: #scrate::metadata::DecodeDifferent::Encode(
						&[ #( #entries, )* ]
					),
				}
			}
		}
	)
}
