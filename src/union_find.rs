#[flux::refined_by(array_size: int)]
#[derive(Debug)]
pub struct UFNode {
    #[flux::field(usize[@array_size])]
    size: usize,
    #[flux::field(usize{v: v <= array_size})]
    parent_index: usize,
    #[flux::field(usize{v: v <= array_size})]
    subtree_size: usize,
}

#[flux::refined_by(array_size: int)]
#[derive(Debug)]
pub struct UnionFind {
    #[flux::field(usize[@array_size])]
    size: usize,
    #[flux::field(Vec<UFNode[@array_size]>)]
    nodes: Vec<UFNode>,
}

#[flux::sig(fn(usize[@size]) -> UnionFind[size])]
fn init_union_find(size: usize) -> UnionFind {
    let mut nodes = Vec::with_capacity(size);
    for i in 0..size {
        nodes.push(UFNode {
            size,
            parent_index: i,
            subtree_size: 0,
        });
    }
    UnionFind {
        size,
        nodes,
    }
}
