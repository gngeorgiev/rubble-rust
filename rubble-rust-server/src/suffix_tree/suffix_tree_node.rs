use std::fmt;

pub struct SuffixTreeNode {
    pub children: Vec<SuffixTreeNode>,
    pub index: Option<i32>,
    pub terminal: bool,
    pub value: String,
}

pub struct SuffixTreeNodeBuilder {
    index: Option<i32>,
    terminal: bool,
    value: String,
}

impl SuffixTreeNodeBuilder {
    fn new() -> SuffixTreeNodeBuilder {
        SuffixTreeNodeBuilder {
            index: None,
            terminal: false,
            value: String::from(""),
        }
    }

    fn index(mut self, i: i32) -> SuffixTreeNodeBuilder {
        self.index = Some(i);
        self
    }

    fn terminal(mut self, t: bool) -> SuffixTreeNodeBuilder {
        self.terminal = t;
        self
    }

    fn value(mut self, v: String) -> SuffixTreeNodeBuilder {
        self.value = v;
        self
    }

    fn finalize(self) -> SuffixTreeNode {
        SuffixTreeNode {
            children: Vec::new(),
            index: self.index,
            terminal: self.terminal,
            value: self.value,
        }
    }
}

impl SuffixTreeNode {
    pub fn default() -> SuffixTreeNode {
        SuffixTreeNode::new().finalize()
    }

    pub fn new() -> SuffixTreeNodeBuilder {
        SuffixTreeNodeBuilder::new()
    }

    fn add_child(&mut self, i: i32, value: String) {
        let node = SuffixTreeNode::new()
            .value(value)
            .index(i as i32)
            .terminal(true)
            .finalize();

        self.children.push(node);
    }

    pub fn build(&mut self, str: String) {
        self.terminal = false;

        let str: String = str.chars().skip(self.value.len()).collect();
        let mut suffix = String::new();

        for (i, char) in str.chars().rev().enumerate() {
            suffix.insert(0, char);

            if self.children.len() == 0 {
                self.add_child(i as i32, suffix.clone());
                continue;
            }

            let res = match self.children.iter_mut().find(|child| {
                suffix.as_str().starts_with(child.value.as_str())
            }) {
                Some(child) => {
                    child.build(suffix.clone());
                    Some(())
                }
                None => None,
            };

            if res.is_none() {
                self.add_child(i as i32, suffix.clone());
            }
        }
    }
}

impl fmt::Display for SuffixTreeNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut terminal_symbol: &str;
        if self.terminal {
            terminal_symbol = "$"
        } else {
            terminal_symbol = ""
        }

        write!(
            f,
            "{} {} {}",
            self.value,
            self.index.unwrap(),
            terminal_symbol
        )
    }
}
