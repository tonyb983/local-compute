// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use seahash::{hash as sea_hash, SeaHasher};
use std::hash::Hasher;
use std::{collections::HashMap, hash::BuildHasherDefault};

pub fn sea_hash_bytes(bytes: &[u8]) -> u64 {
    sea_hash(bytes)
}

pub fn default_hash_bytes(bytes: &[u8]) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    hasher.write(bytes);
    hasher.finish()
}

pub fn default_hashmap<K, V>() -> HashMap<K, V> {
    HashMap::new()
}

pub type SeaHashBuilder = BuildHasherDefault<SeaHasher>;

pub fn sea_hashmap<K, V>() -> HashMap<K, V, SeaHashBuilder> {
    HashMap::<K, V, SeaHashBuilder>::default()
}
