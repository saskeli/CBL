#![allow(dead_code)]

pub use cxx::UniquePtr;
pub use tv16::*;
pub use tv24::*;
pub use tv32::*;

#[cxx::bridge]
mod tv32 {
    unsafe extern "C++" {
        include!("CBL/cxx/tiered_vec.h");

        type TieredVec32;
        fn new_tiered_vec_32() -> UniquePtr<TieredVec32>;
        fn len(&self) -> usize;
        fn is_empty(&self) -> bool;
        fn capacity(&self) -> usize;
        fn get(&self, idx: usize) -> u32;
        fn update(&self, idx: usize, elem: u32) -> u32;
        fn insert(&self, idx: usize, elem: u32);
        fn remove(&self, idx: usize);
        fn insert_sorted(&self, elem: u32);
        fn contains_sorted(&self, elem: u32) -> bool;
    }
}

#[cxx::bridge]
mod tv24 {
    unsafe extern "C++" {
        include!("CBL/cxx/tiered_vec.h");

        type TieredVec24;
        fn new_tiered_vec_24() -> UniquePtr<TieredVec24>;
        fn len(&self) -> usize;
        fn is_empty(&self) -> bool;
        fn capacity(&self) -> usize;
        fn get(&self, idx: usize) -> u32;
        fn update(&self, idx: usize, elem: u32) -> u32;
        fn insert(&self, idx: usize, elem: u32);
        fn remove(&self, idx: usize);
        fn insert_sorted(&self, elem: u32);
        fn contains_sorted(&self, elem: u32) -> bool;
    }
}

#[cxx::bridge]
mod tv16 {
    unsafe extern "C++" {
        include!("CBL/cxx/tiered_vec.h");

        type TieredVec16;
        fn new_tiered_vec_16() -> UniquePtr<TieredVec16>;
        fn len(&self) -> usize;
        fn is_empty(&self) -> bool;
        fn capacity(&self) -> usize;
        fn get(&self, idx: usize) -> u16;
        fn update(&self, idx: usize, elem: u16) -> u16;
        fn insert(&self, idx: usize, elem: u16);
        fn remove(&self, idx: usize);
        fn insert_sorted(&self, elem: u16);
        fn contains_sorted(&self, elem: u16) -> bool;
    }
}
