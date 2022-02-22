// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
use std::collections::HashSet;

mod logger;

pub use logger::{LogLevel, Logger};

use crate::ComputeFunction;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BuiltinFunction {
    Logger,
}

impl BuiltinFunction {
    pub fn create(self) -> Box<dyn ComputeFunction> {
        match self {
            Self::Logger => Box::new(Logger::default()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BuiltinFunctionList(HashSet<BuiltinFunction>);

impl BuiltinFunctionList {
    pub fn new() -> Self {
        Self(HashSet::new())
    }

    pub fn add(&mut self, function: BuiltinFunction) -> bool {
        self.0.insert(function)
    }

    pub fn add_multiple(&mut self, functions: &[BuiltinFunction]) -> usize {
        let mut success = 0;
        for function in functions {
            if self.add(*function) {
                success += 1;
            }
        }

        success
    }

    pub fn contains(&self, function: BuiltinFunction) -> bool {
        self.0.contains(&function)
    }

    pub fn create_all(&self) -> Vec<Box<dyn ComputeFunction>> {
        self.0.iter().map(|function| function.create()).collect()
    }
}

impl<T> From<T> for BuiltinFunctionList
where
    T: IntoIterator<Item = BuiltinFunction>,
{
    fn from(iter: T) -> Self {
        let mut list = Self::new();
        for function in iter {
            list.add(function);
        }

        list
    }
}
