#![feature(register_tool)]
#![register_tool(flux)]
#![feature(custom_inner_attributes)]

use union_find::*;

mod union_find;

#[flux::trusted]
fn main() {
    let mut uf = init_union_find(10);
    union(&mut uf, 0, 1);
    union(&mut uf, 1, 2);
    union(&mut uf, 8, 9);
    assert!(find_root(&mut uf, 0) == find_root(&mut uf, 2));
    assert!(find_root(&mut uf, 0) != find_root(&mut uf, 8));
    assert!(find_root(&mut uf, 8) == find_root(&mut uf, 9));
}
