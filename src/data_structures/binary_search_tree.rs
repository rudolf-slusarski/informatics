use std::cmp::Ordering;

struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    key: u32,
    count: u32,
}

impl Node {
    pub fn new(key: u32) -> Self {
        Self {
            left: None,
            right: None,
            key,
            count: 1,
        }
    }

    pub fn insert(&mut self, n: u32) {
        match self.key.cmp(&n) {
            Ordering::Equal => self.count += 1,
            Ordering::Less => match &mut self.right {
                Some(x) => x.insert(n),
                None => self.right = Some(box Node::new(n)),
            },
            Ordering::Greater => match &mut self.left {
                Some(x) => x.insert(n),
                None => self.left = Some(box Node::new(n)),
            },
        }
    }

    pub fn search(&self, n: u32) -> Option<&Node> {
        match self.key.cmp(&n) {
            Ordering::Equal => Some(&self),
            Ordering::Greater => match &self.left {
                Some(x) => x.search(n),
                None => None,
            },
            Ordering::Less => match &self.right {
                Some(x) => x.search(n),
                None => None,
            },
        }
    }

    // pub fn remove(&mut self, n: u32) { }
}
