use suffix_tree_node::SuffixTreeNode;

pub struct SuffixTree {
    root: SuffixTreeNode
}

impl SuffixTree {
    pub fn new() -> SuffixTree {
        SuffixTree {
            root: SuffixTreeNode::default()
        }
    }

    pub fn build(&mut self, str: String) {
        self.root.build(str);
    }
}