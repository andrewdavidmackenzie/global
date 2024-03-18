#![feature(lazy_cell)]

use std::cell::LazyCell;

/*struct Global<T> {
    inner: LazyCell<T>
}

impl<T> Global<T> {
    pub const fn new(value: T) -> Self {
        Self {
            inner: LazyCell::new(|| {
                println!("initializing");
                value
            })
        }
    }
}

const MEANING: Global<u32> = Global::new(42);
*/

const LAZY: LazyCell<i32> = LazyCell::new(|| 92 );

fn main() {
    println!("FOO {}", *LAZY);
}