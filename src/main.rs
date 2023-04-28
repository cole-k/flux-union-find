#![feature(register_tool)]
#![register_tool(flux)]
#![feature(custom_inner_attributes)]

use union_find::*;

mod union_find;

#[flux::trusted]
fn main() {
    let mut uf = init_union_find(10);
    println!("root of 0: {}", find_root(&mut uf, 0));
    println!("root of 3: {}", find_root(&mut uf, 3));
}
