#![allow(unused)]

use std::time::{Duration, Instant};

type Rt<T = ()> = Result<T, Box<dyn std::error::Error>>;

rcstruct::rcstruct! {
    pub struct GUI {}

    impl {
        pub new() -> Rt<Self> {
            {}
        }

        pub fn foo(&self, a: u32, yall: Box<u32>) -> () {}

        fn multiarg(&self, a: u32, b: u32, c: u32) -> u32 {
            a + b * c
        }
    }
}

fn main() -> Rt {
    let gui = GUI::new()?;
    Ok(())
}
