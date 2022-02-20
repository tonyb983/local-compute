// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
#![allow(dead_code)]

use libloading::Library;

const BAD_ADDER_DLL: &str =
    "C:\\Tony\\Code\\Rust\\local-compute\\ext\\out\\bad_adder\\bad_adder.dll";
const GOOD_ADDER_DLL: &str =
    "C:\\Tony\\Code\\Rust\\local-compute\\ext\\out\\good_adder\\good_adder.dll";

type AddFunc = unsafe fn(isize, isize) -> isize;

fn load_and_call_adder(lib_path: &str, adder_func: &str, a: isize, b: isize) -> Option<isize> {
    unsafe {
        match Library::new(lib_path) {
            Ok(lib) => match lib.get::<AddFunc>(adder_func.as_bytes()) {
                Ok(func) => Some(func(a, b)),
                Err(_) => None,
            },
            Err(_) => None,
        }
    }
}

fn execute(a: isize, b: isize) -> (Option<isize>, Option<isize>) {
    let good_output = load_and_call_adder(GOOD_ADDER_DLL, "add", a, b);
    let bad_output = load_and_call_adder(BAD_ADDER_DLL, "add", a, b);

    (good_output, bad_output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let (good, bad) = execute(2, 2);
        println!("Good: {:?}", good);
        println!("Bad: {:?}", bad);
        assert_eq!(good, Some(4));
        assert_eq!(bad, Some(0));
    }
}
