#[flux::refined_by(array_size: int)]
#[derive(Debug)]
pub struct UFNode {
    #[flux::field(usize[@array_size])]
    _size: usize,
    #[flux::field(usize{v: v <= array_size})]
    parent_index: usize,
    #[flux::field(usize{v: v <= array_size})]
    subtree_size: usize,
}

#[flux::refined_by(array_size: int)]
#[derive(Debug)]
pub struct UnionFind {
    #[flux::field(usize[@array_size])]
    _size: usize,
    #[flux::field(Vec<UFNode[@array_size]>)]
    nodes: Vec<UFNode>,
}

#[flux::trusted]
#[flux::sig(fn(&UnionFind[@size], usize{v: v <= size}) -> &UFNode[size])]
fn get_node(uf: &UnionFind, index: usize) -> &UFNode {
    uf.nodes.get(index).unwrap()
}

#[flux::trusted]
#[flux::sig(fn(&mut UnionFind[@size], usize{v: v <= size}) -> &mut UFNode[size])]
fn get_mut_node(uf: &mut UnionFind, index: usize) -> &mut UFNode {
    uf.nodes.get_mut(index).unwrap()
}

#[flux::sig(fn(usize[@size]) -> UnionFind[size])]
pub fn init_union_find(size: usize) -> UnionFind {
    let mut nodes = Vec::with_capacity(size);
    for i in 0..size {
        nodes.push(UFNode {
            _size: size,
            parent_index: i,
            subtree_size: 0,
        });
    }
    UnionFind {
        _size: size,
        nodes,
    }
}

#[flux::sig(fn(&mut UnionFind[@size], node_index: usize{v: v <= size}) -> usize{v: v <= size})]
pub fn find_root(uf: &mut UnionFind, node_index: usize) -> usize {
    let parent_index = get_node(uf, node_index).parent_index;
    if parent_index == node_index {
        return node_index;
    }
    let root = find_root(uf, parent_index);
    get_mut_node(uf, node_index).parent_index = root;
    return root;
}

#[flux::sig(fn(&mut UnionFind[@size], node_index1: usize{v: v <= size}, node_index2: usize{v: v <= size}))]
pub fn union(uf: &mut UnionFind, node_index1: usize, node_index2: usize) {
    let root1 = find_root(uf, node_index1);
    let root2 = find_root(uf, node_index2);

    if root1 == root2 {
        return;
    }

    if get_node(uf, node_index1).subtree_size < get_node(uf, node_index2).subtree_size {
        get_mut_node(uf, node_index2).parent_index = node_index1;
        get_mut_node(uf, node_index1).subtree_size = get_node(uf, node_index1).subtree_size + get_node(uf, node_index2).subtree_size;
    } else {
        get_mut_node(uf, node_index1).parent_index = node_index2;
        get_mut_node(uf, node_index2).subtree_size = get_node(uf, node_index2).subtree_size + get_node(uf, node_index1).subtree_size;
    }
}
