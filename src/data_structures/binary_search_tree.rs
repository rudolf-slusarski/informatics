use std::{cmp::Ordering, mem::replace};

#[derive(Debug)]
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
            Ordering::Equal => Some(self),
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

    pub fn print(&self, mut offset: usize) {
        offset += 1;
        println!("{:offset$}key: {} count: {}", " ", self.key, self.count);
        if let Some(x) = &self.left {
            x.print(offset)
        }
        if let Some(x) = &self.right {
            x.print(offset)
        }
    }

    pub fn remove(mut this: Box<Self>, n: &u32) -> Option<Box<Node>> {
        match this.key.cmp(n) {
            Ordering::Less => {
                if let Some(left) = this.left.take() {
                    this.left = Self::remove(left, n)
                }
                Some(this)
            }
            Ordering::Greater => {
                if let Some(right) = this.right.take() {
                    this.right = Self::remove(right, n)
                }
                Some(this)
            }
            Ordering::Equal => match (this.left.take(), this.right.take()) {
                (None, None) => None,
                (None, Some(right)) => Some(right),
                (Some(left), None) => Some(left),
                (Some(mut left), Some(right)) => {
                    if let Some(mut rightmost) = left.rightmost() {
                        rightmost.left = Some(left);
                        rightmost.right = Some(right);
                        Some(rightmost)
                    } else {
                        left.right = Some(right);
                        Some(left)
                    }
                },
            },
        }
    }

    fn rightmost(&mut self) -> Option<Box<Node>> {
        match self.right {
            Some(ref mut right) => {
                if let Some(t) = right.rightmost() {
                    Some(t)
                } else {
                    let mut r = self.right.take();
                    if let Some(ref mut r) = r {
                        self.right = replace(&mut r.left, None);
                    }
                    r
                }
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut tree: Node = Node::new(7);
        tree.insert(4);
        tree.insert(3);
        tree.insert(22);
        tree.insert(2);
        tree.insert(21);
        tree.insert(27);
        tree.insert(5);
        tree.insert(23);
        tree.insert(22);
        tree.insert(22);
        tree.insert(22);
        tree.insert(24);
        tree.insert(24);
        tree.insert(2);

        tree.print(0);
        println!();
        // tree.remove(24);
        // tree.remove(24);

        tree.insert(64);
        tree.print(0);
        // println!("{:?}", tree);

        println!("\n\n");
        println!(
            "30: {:?}\n2: {:?}\n24: {:?}",
            tree.search(30),
            tree.search(2),
            tree.search(24)
        );

        println!("{:?}", tree);

        println!();
        tree = *Node::remove(box tree, &7).unwrap();
        tree.print(0);
    }
}
