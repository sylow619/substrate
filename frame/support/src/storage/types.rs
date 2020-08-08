// This file is part of Substrate.

// Copyright (C) 2017-2020 Parity Technologies (UK) Ltd.
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

//! Types to use instead of traits

use codec::{FullEncode, FullCodec};
use crate::traits::{GetDefault, StorageInstance};
use frame_metadata::{
	StorageEntryMetadata, DecodeDifferentStr, DefaultByte, StorageEntryType, DecodeDifferentArray,
	DecodeDifferent, DefaultByteGetter, StorageEntryModifier,
};

// TODO TODO: maybe rename this one with Default so that it is less confusing for user.
impl<Prefix, Value, OnEmpty> super::generator::StorageValue<Value> for
	StorageValueType<Prefix, Value, Value, OnEmpty>
where
	Prefix: StorageInstance,
	Value: FullCodec,
	OnEmpty: crate::traits::Get<Value>
{
	type Query = Value;
	fn module_prefix() -> &'static [u8] {
		<Prefix::I as crate::traits::Instance>::PREFIX.as_bytes()
	}
	fn storage_prefix() -> &'static [u8] {
		Prefix::STORAGE_PREFIX.as_bytes()
	}
	fn from_optional_value_to_query(v: Option<Value>) -> Self::Query {
		v.unwrap_or_else(OnEmpty::get)
	}
	fn from_query_to_optional_value(v: Self::Query) -> Option<Value> {
		Some(v)
	}
}

impl<Prefix, Value, OnEmpty> super::generator::StorageValue<Value> for
	StorageValueType<Prefix, Value, Option<Value>, OnEmpty>
where
	Prefix: StorageInstance,
	Value: FullCodec,
	OnEmpty: crate::traits::Get<Value>
{
	type Query = Option<Value>;
	fn module_prefix() -> &'static [u8] {
		<Prefix::I as crate::traits::Instance>::PREFIX.as_bytes()
	}
	fn storage_prefix() -> &'static [u8] {
		Prefix::STORAGE_PREFIX.as_bytes()
	}
	fn from_optional_value_to_query(v: Option<Value>) -> Self::Query {
		v
	}
	fn from_query_to_optional_value(v: Self::Query) -> Option<Value> {
		v
	}
}

pub struct StorageMapType<Prefix, Hasher, Key, Value, Query=Option<Value>, OnEmpty=GetDefault>(
	core::marker::PhantomData<(Prefix, Hasher, Key, Value, Query, OnEmpty)>
);

impl<Prefix, Hasher, Key, Value, Query, OnEmpty>
	StorageMapType<Prefix, Hasher, Key, Value, Query, OnEmpty>
where
	Prefix: StorageInstance,
	Hasher: crate::hash::StorageHasher,
	Value: FullCodec + 'static,
	OnEmpty: crate::traits::Get<Value> + 'static,
{
	#[doc(hidden)]
	pub fn storage_entry_metadata_builder(
		key_metadata: &'static str,
		value_metadata: &'static str,
		doc: &'static [&'static str],
	) -> StorageEntryMetadata {
		StorageEntryMetadata {
			name: DecodeDifferentStr::Encode(Prefix::STORAGE_PREFIX),
			modifier: StorageEntryModifier::Default,
			ty: StorageEntryType::Map {
				hasher: Hasher::METADATA,
				key: DecodeDifferentStr::Encode(key_metadata),
				value: DecodeDifferentStr::Encode(value_metadata),
				unused: false,
			},
			default: DecodeDifferent::Encode(DefaultByteGetter(&OnEmptyGetter::<Value, OnEmpty>(core::marker::PhantomData))),
			documentation: DecodeDifferentArray::Encode(doc),
		}
	}
}

impl<Prefix, Hasher, Key, Value, OnEmpty> super::generator::StorageMap<Key, Value> for
	StorageMapType<Prefix, Hasher, Key, Value, Value, OnEmpty>
where
	Prefix: StorageInstance,
	Hasher: crate::hash::StorageHasher,
	Key: FullEncode,
	Value: FullCodec,
	OnEmpty: crate::traits::Get<Value>
{
	type Query = Value;
	type Hasher = Hasher;
	fn module_prefix() -> &'static [u8] {
		<Prefix::I as crate::traits::Instance>::PREFIX.as_bytes()
	}
	fn storage_prefix() -> &'static [u8] {
		Prefix::STORAGE_PREFIX.as_bytes()
	}
	fn from_optional_value_to_query(v: Option<Value>) -> Self::Query {
		v.unwrap_or_else(OnEmpty::get)
	}
	fn from_query_to_optional_value(v: Self::Query) -> Option<Value> {
		Some(v)
	}
}

impl<Prefix, Hasher, Key, Value, OnEmpty> super::generator::StorageMap<Key, Value> for
	StorageMapType<Prefix, Hasher, Key, Value, Option<Value>, OnEmpty>
where
	Prefix: StorageInstance,
	Hasher: crate::hash::StorageHasher,
	Key: FullEncode,
	Value: FullCodec,
	OnEmpty: crate::traits::Get<Value>
{
	type Query = Option<Value>;
	type Hasher = Hasher;
	fn module_prefix() -> &'static [u8] {
		<Prefix::I as crate::traits::Instance>::PREFIX.as_bytes()
	}
	fn storage_prefix() -> &'static [u8] {
		Prefix::STORAGE_PREFIX.as_bytes()
	}
	fn from_optional_value_to_query(v: Option<Value>) -> Self::Query {
		v
	}
	fn from_query_to_optional_value(v: Self::Query) -> Option<Value> {
		v
	}
}

pub struct StorageDoubleMapType<
	Prefix, Hasher1, Key1, Hasher2, Key2, Value, Query=Option<Value>, OnEmpty=GetDefault
>(
	core::marker::PhantomData<(Prefix, Hasher1, Key1, Hasher2, Key2, Value, Query, OnEmpty)>
);

impl<Prefix, Hasher1, Key1, Hasher2, Key2, Value, Query, OnEmpty>
	StorageDoubleMapType<Prefix, Hasher1, Key1, Hasher2, Key2, Value, Query, OnEmpty>
where
	Prefix: StorageInstance,
	Hasher1: crate::hash::StorageHasher,
	Hasher2: crate::hash::StorageHasher,
	Value: FullCodec + 'static,
	OnEmpty: crate::traits::Get<Value> + 'static,
{
	#[doc(hidden)]
	pub fn storage_entry_metadata_builder(
		key1_metadata: &'static str,
		key2_metadata: &'static str,
		value_metadata: &'static str,
		doc: &'static [&'static str],
	) -> StorageEntryMetadata {
		StorageEntryMetadata {
			name: DecodeDifferentStr::Encode(Prefix::STORAGE_PREFIX),
			modifier: StorageEntryModifier::Default,
			ty: StorageEntryType::DoubleMap {
				hasher: Hasher1::METADATA,
				key2_hasher: Hasher2::METADATA,
				key1: DecodeDifferentStr::Encode(key1_metadata),
				key2: DecodeDifferentStr::Encode(key2_metadata),
				value: DecodeDifferentStr::Encode(value_metadata),
			},
			default: DecodeDifferent::Encode(DefaultByteGetter(&OnEmptyGetter::<Value, OnEmpty>(core::marker::PhantomData))),
			documentation: DecodeDifferentArray::Encode(doc),
		}
	}
}

impl<Prefix, Hasher1, Key1, Hasher2, Key2, Value, OnEmpty>
	super::generator::StorageDoubleMap<Key1, Key2, Value> for
	StorageDoubleMapType<Prefix, Hasher1, Key1, Hasher2, Key2, Value, Value, OnEmpty>
where
	Prefix: StorageInstance,
	Hasher1: crate::hash::StorageHasher,
	Hasher2: crate::hash::StorageHasher,
	Key1: FullEncode,
	Key2: FullEncode,
	Value: FullCodec,
	OnEmpty: crate::traits::Get<Value>
{
	type Query = Value;
	type Hasher1 = Hasher1;
	type Hasher2 = Hasher2;
	fn module_prefix() -> &'static [u8] {
		<Prefix::I as crate::traits::Instance>::PREFIX.as_bytes()
	}
	fn storage_prefix() -> &'static [u8] {
		Prefix::STORAGE_PREFIX.as_bytes()
	}
	fn from_optional_value_to_query(v: Option<Value>) -> Self::Query {
		v.unwrap_or_else(OnEmpty::get)
	}
	fn from_query_to_optional_value(v: Self::Query) -> Option<Value> {
		Some(v)
	}
}

impl<Prefix, Hasher1, Key1, Hasher2, Key2, Value, OnEmpty>
	super::generator::StorageDoubleMap<Key1, Key2, Value> for
	StorageDoubleMapType<Prefix, Hasher1, Key1, Hasher2, Key2, Value, Option<Value>, OnEmpty>
where
	Prefix: StorageInstance,
	Hasher1: crate::hash::StorageHasher,
	Hasher2: crate::hash::StorageHasher,
	Key1: FullEncode,
	Key2: FullEncode,
	Value: FullCodec,
	OnEmpty: crate::traits::Get<Value>
{
	type Query = Option<Value>;
	type Hasher1 = Hasher1;
	type Hasher2 = Hasher2;
	fn module_prefix() -> &'static [u8] {
		<Prefix::I as crate::traits::Instance>::PREFIX.as_bytes()
	}
	fn storage_prefix() -> &'static [u8] {
		Prefix::STORAGE_PREFIX.as_bytes()
	}
	fn from_optional_value_to_query(v: Option<Value>) -> Self::Query {
		v
	}
	fn from_query_to_optional_value(v: Self::Query) -> Option<Value> {
		v
	}
}

struct OnEmptyGetter<Value, OnEmpty>(core::marker::PhantomData<(Value, OnEmpty)>);
impl<Value: FullCodec, OnEmpty: crate::traits::Get<Value>> DefaultByte for OnEmptyGetter<Value, OnEmpty> {
	fn default_byte(&self) -> sp_std::vec::Vec<u8> {
		OnEmpty::get().encode()
	}
}
unsafe impl <Value, OnEmpty: crate::traits::Get<Value>> Send for OnEmptyGetter<Value, OnEmpty> {}
unsafe impl <Value, OnEmpty: crate::traits::Get<Value>> Sync for OnEmptyGetter<Value, OnEmpty> {}

pub struct StorageValueType<Prefix, Value, Query=Option<Value>, OnEmpty=GetDefault>(
	core::marker::PhantomData<(Prefix, Value, Query, OnEmpty)>
);

impl<Prefix, Value, Query, OnEmpty> StorageValueType<Prefix, Value, Query, OnEmpty> where
	Prefix: StorageInstance,
	Value: FullCodec + 'static,
	OnEmpty: crate::traits::Get<Value> + 'static
{
	pub const NAME: &'static str = Prefix::STORAGE_PREFIX;
	pub const DEFAULT: DefaultByteGetter =
		DefaultByteGetter(&OnEmptyGetter::<Value, OnEmpty>(core::marker::PhantomData));
}

impl<Prefix, Hasher, Key, Value, Query, OnEmpty>
	StorageMapType<Prefix, Hasher, Key, Value, Query, OnEmpty>
where
	Hasher: crate::hash::StorageHasher,
	Prefix: StorageInstance,
	Value: FullCodec + 'static,
	OnEmpty: crate::traits::Get<Value> + 'static
{
	pub const HASHER: frame_metadata::StorageHasher = Hasher::METADATA;
	pub const NAME: &'static str = Prefix::STORAGE_PREFIX;
	pub const DEFAULT: DefaultByteGetter =
		DefaultByteGetter(&OnEmptyGetter::<Value, OnEmpty>(core::marker::PhantomData));
}

impl<Prefix, Hasher1, Hasher2, Key1, Key2, Value, Query, OnEmpty>
	StorageDoubleMapType<Prefix, Hasher1, Key1, Hasher2, Key2, Value, Query, OnEmpty>
where
	Hasher1: crate::hash::StorageHasher,
	Hasher2: crate::hash::StorageHasher,
	Prefix: StorageInstance,
	Value: FullCodec + 'static,
	OnEmpty: crate::traits::Get<Value> + 'static
{
	pub const HASHER1: frame_metadata::StorageHasher = Hasher1::METADATA;
	pub const HASHER2: frame_metadata::StorageHasher = Hasher2::METADATA;
	pub const NAME: &'static str = Prefix::STORAGE_PREFIX;
	pub const DEFAULT: DefaultByteGetter =
		DefaultByteGetter(&OnEmptyGetter::<Value, OnEmpty>(core::marker::PhantomData));
}

pub trait StorageMetadataMofidierGetter {
	const MODIFIER: StorageEntryModifier;
}

impl<Prefix, Value, OnEmpty> StorageMetadataMofidierGetter for
	StorageValueType<Prefix, Value, Option<Value>, OnEmpty>
{
	const MODIFIER: StorageEntryModifier = StorageEntryModifier::Optional;
}

impl<Prefix, Value, OnEmpty> StorageMetadataMofidierGetter for
	StorageValueType<Prefix, Value, Value, OnEmpty>
{
	const MODIFIER: StorageEntryModifier = StorageEntryModifier::Default;
}

impl<Prefix, Hasher, Key, Value, OnEmpty> StorageMetadataMofidierGetter for
	StorageMapType<Prefix, Hasher, Key, Value, Option<Value>, OnEmpty>
{
	const MODIFIER: StorageEntryModifier = StorageEntryModifier::Optional;
}

impl<Prefix, Hasher, Key, Value, OnEmpty> StorageMetadataMofidierGetter for
	StorageMapType<Prefix, Hasher, Key, Value, Value, OnEmpty>
{
	const MODIFIER: StorageEntryModifier = StorageEntryModifier::Default;
}

impl<Prefix, Hasher1, Key1, Hasher2, Key2, Value, OnEmpty> StorageMetadataMofidierGetter for
	StorageDoubleMapType<Prefix, Hasher1, Key1, Hasher2, Key2, Value, Option<Value>, OnEmpty>
{
	const MODIFIER: StorageEntryModifier = StorageEntryModifier::Optional;
}

impl<Prefix, Hasher1, Key1, Hasher2, Key2, Value, OnEmpty> StorageMetadataMofidierGetter for
	StorageDoubleMapType<Prefix, Hasher1, Key1, Hasher2, Key2, Value, Value, OnEmpty>
{
	const MODIFIER: StorageEntryModifier = StorageEntryModifier::Default;
}
