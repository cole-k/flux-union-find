#![feature(register_tool)]
#![register_tool(flux)]
#![feature(custom_inner_attributes)]

use union_find::*;

mod union_find;

#[flux::trusted]
fn main() {
}
