/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/


use std::cmp::Ordering;
use std::fmt::Debug;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord+ Clone+Debug,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T)  {
        //TODO
        match &mut self.root {
            Some(r)=>{
                (*r).insert(value);
            },
            None=>{
                let root = TreeNode::new(value);
                self.root = Some(Box::new(root)) ;
            }
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        match &self.root {
            Some(root)=>{
                root.search(value)
            },
            None=>{
                false
            }
        }
        // true
    }
}

impl<T> TreeNode<T>
where
    T: Ord+ Clone+Debug,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
        if value < self.value{
            match &mut self.left {
                Some(left)=>{
                    (*left).insert(value);
                },
                _=>{
                    let new_node = TreeNode::new(value);
                    self.left = Option::Some(Box::new(new_node));
                }
            }
        }else if value > self.value{
            match &mut self.right {
                Some(right)=>{
                    (*right).insert(value);
                },
                _=>{
                    let new_node = TreeNode::new(value);
                    self.right = Option::Some(Box::new(new_node));
                }
            }
        }
    }

    fn search(&self, value: T) -> bool{
        if value < self.value {
            match &self.left {
                Some(left)=>{
                    left.search(value)
                },
                None=>{
                    false
                }
            }
        }else if value > self.value{
            match &self.right {
                Some(right)=>{
                    right.search(value)
                },
                None=>{
                    false
                }
            }
        }else {
            true
        }
        // !unimplemented!()
    }
}

fn main() {
    let array = [1,2,4,3,7,6,5,1,4];
    let mut tree: BinarySearchTree <u32> = BinarySearchTree::new();
    for i in 0..array.len(){
        tree.insert(array[i]);
    }

    println!("{:#?}", tree);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


