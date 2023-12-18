# Problem Statement

Write a function called `find_palindromes` that takes a vector of strings as an argument and returns a vector of strings, where each string is a palindrome that can be formed by concatenating some of the words in the original vector. A palindrome is a word or phrase that is the same when read forwards or backwards, such as "racecar" or "madam". The function should ignore the case of the letters, and the order of the strings in the output vector does not matter.

## Test Cases

- Input: `vec!["mad", "am", "dam", "race", "car", "a"]`
- Output: `vec!["madam", "racecar", "a"]`

- Input: `vec!["eye", "level", "pop", "noon", "wow", "see"]`
- Output: `vec!["eye", "level", "pop", "noon", "wow", "sees", "eyese", "levellevel", "popop", "noonnoon", "wowow"]`

- Input: `vec!["red", "rum", "murder", "der", "um", "ed"]`
- Output: `vec!["redrum", "murder", "dered", "ummu", "ed"]`

## Code Template

```rust
// Write your code here
fn find_palindromes(words: Vec<&str>) -> Vec<String> {
    // TODO: implement this function
    unimplemented!()
}

// Do not modify the code below
fn main() {
    let words = vec!["mad", "am", "dam", "race", "car", "a"];
    let result = find_palindromes(words);
    println!("{:?}", result);
}
```