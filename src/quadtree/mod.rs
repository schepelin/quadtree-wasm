#[derive(Debug, Eq, PartialEq)]
struct Point(usize, usize);

pub struct Node {
    top_left: Point,
    bottom_right: Point,
}

pub struct QuadTree {
    node_capacity: usize,
    root: Node,
}

impl QuadTree {

    fn new(node_capacity: usize, grid_width: usize, grid_height: usize) -> Self {
        // Init a tree struct with passed capacity per node
        // Create a tree's root with empty children and points vector
        return QuadTree{
            node_capacity,
            root:Node{top_left: Point(0, 0), bottom_right: Point(grid_width, grid_height)}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_initialization() {
        let width: usize = 500;
        let height: usize = 300;
        let tree = QuadTree::new(10, width, height);

        assert_eq!(tree.node_capacity, 10);
        let root = tree.root;
        assert_eq!(root.top_left, Point(0,0));
        assert_eq!(root.bottom_right, Point(width, height));
        // TODO: check children list
        // check points vector

    }
}

