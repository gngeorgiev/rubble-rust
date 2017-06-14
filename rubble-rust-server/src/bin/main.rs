extern crate suffix_tree;

use suffix_tree::SuffixTree;

fn main() {
    let pattern = String::from("BANANA");

    let mut tree = SuffixTree::new();
    tree.build(pattern);
}
