/*
   Copyright 2016 Lipka Boldizsár

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

     http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/
extern crate hunspell_sys;

use std::ffi::{CString, CStr};
use std::ptr::null_mut;
use std::os::raw::c_char;

use hunspell_sys::*;

#[test]
fn create_and_destroy() {
    let hs = Hunspell::new("tests/fixtures/reduced.aff",
                           "tests/fixtures/reduced.dic");
}

#[test]
fn check() {
    let hs = Hunspell::new("tests/fixtures/reduced.aff",
                           "tests/fixtures/reduced.dic");
    assert!(hs.check("cats"));
    assert!(!hs.check("nocats"));
}

#[test]
fn suggest() {
    let hs = Hunspell::new("tests/fixtures/reduced.aff",
                           "tests/fixtures/reduced.dic");
    assert!(hs.suggest("progra").len() > 0);
}

#[test]
fn stem() {
    let hs = Hunspell::new("tests/fixtures/reduced.aff",
                           "tests/fixtures/reduced.dic");
    let cat_stem = hs.stem("cats");
    assert!(cat_stem[0] == "cat");
}

type CStringList = *mut *mut i8;

pub struct Hunspell {
    handle: *mut Hunhandle
}

macro_rules! extract_vec {
    ( $fname:ident, $handle:expr, $( $arg:expr ),* ) => {
        {
            let mut result = Vec::new();
            unsafe {
                let mut list = null_mut();
                let n = $fname($handle, &mut list, $( $arg ),*) as isize;
                if n != 0 {
                    for i in 0..n {
                        let item = CStr::from_ptr(*list.offset(i));
                        result.push(String::from(item.to_str().unwrap()));
                    }
                    Hunspell_free_list($handle, &mut list, n as i32);
                }
            }
            result
        }
    }
}

impl Hunspell {
    pub fn new(affpath: &str, dicpath: &str) -> Hunspell {
        let affpath = CString::new(affpath).unwrap();
        let dicpath = CString::new(dicpath).unwrap();
        unsafe {
            Hunspell {
                handle: Hunspell_create(affpath.as_ptr(), dicpath.as_ptr())
            }
        }
    }

    pub fn new_with_key(affpath: &str, dicpath: &str, key: &str) -> Hunspell {
        let affpath = CString::new(affpath).unwrap();
        let dicpath = CString::new(dicpath).unwrap();
        let key = CString::new(key).unwrap();
        unsafe {
            Hunspell {
                handle: Hunspell_create_key(affpath.as_ptr(), dicpath.as_ptr(),
                                            key.as_ptr())
            }
        }
    }

    pub fn check(&self, word: &str) -> bool {
        let word = CString::new(word).unwrap();
        unsafe {
            Hunspell_spell(self.handle, word.as_ptr()) == 1
        }
    }

    pub fn suggest(&self, word: &str) -> Vec<String> {
        let word = CString::new(word).unwrap();
        extract_vec!(Hunspell_suggest, self.handle, word.as_ptr())
    }

    pub fn analyze(&self, word: &str) -> Vec<String> {
        let word = CString::new(word).unwrap();
        extract_vec!(Hunspell_analyze, self.handle, word.as_ptr())
    }

    pub fn stem(&self, word: &str) -> Vec<String> {
        let word = CString::new(word).unwrap();
        extract_vec!(Hunspell_stem, self.handle, word.as_ptr())
    }

    pub fn generate(&self, word1: &str, word2: &str) -> Vec<String> {
        let word1 = CString::new(word1).unwrap();
        let word2 = CString::new(word2).unwrap();
        extract_vec!(Hunspell_generate, self.handle, word1.as_ptr(), word2.as_ptr())
    }
}

impl Drop for Hunspell {
    fn drop(&mut self) {
        unsafe {
            Hunspell_destroy(self.handle);
        }
    }
}
