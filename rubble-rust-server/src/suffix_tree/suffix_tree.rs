use std::fmt;

use suffix_tree_node::SuffixTreeNode;

pub struct SuffixTree {
    root: SuffixTreeNode,
}

impl SuffixTree {
    pub fn new() -> SuffixTree {
        SuffixTree { root: SuffixTreeNode::default() }
    }

    pub fn build(&mut self, str: String) {
        self.root.build(str);
    }
}

impl fmt::Display for SuffixTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        let iterate = |node: &SuffixTreeNode| {
//            let mut str = String::new();
//            for child in &node.children {
//                str.push_str(" - ");
//                str.push_str(child.value.as_str());
//            }
//
//            str.push_str("\r\n");
//            write!(f, "{}", str);
//            for child in &node.children {
//                iterate(&child);
//            }
//        };
//
//        iterate(&self.root);
        Ok(())
    }
}
