# Problem Statement

Write a function called `sum_tree` that takes a box pointer to a binary tree node as an argument and returns the sum of all the values in the tree. A binary tree is a data structure that consists of nodes that store a value and two pointers to its left and right children. You can use box pointers to allocate nodes on the heap and link them together. Your function should handle the case where the input is None, which represents an empty tree.

## Test Cases

- Input: `Some(Box::new(TreeNode { value: 1, left: None, right: None }))`
- Output: `1`

- Input: `Some(Box::new(TreeNode { value: 2, left: Some(Box::new(TreeNode { value: 3, left: None, right: None })), right: Some(Box::new(TreeNode { value: 4, left: None, right: None })) }))`
- Output: `9`

- Input: `None`
- Output: `0`

## Code Template

```rust
// Define a struct for tree nodes
struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

// Write your code here
fn sum_tree(root: Option<Box<TreeNode>>) -> i32 {
    // TODO: implement this function
    unimplemented!()
}

// Do not modify the code below
fn main() {
    let root = Some(Box::new(TreeNode {
        value: 2,
        left: Some(Box::new(TreeNode {
            value: 3,
            left: None,
            right: None,
        })),
        right: Some(Box::new(TreeNode {
            value: 4,
            left: None,
            right: None,
        })),
    }));
    let result = sum_tree(root);
    println!("{}", result);
}
```
