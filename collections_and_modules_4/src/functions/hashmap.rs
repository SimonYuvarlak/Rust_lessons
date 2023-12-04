    use std::collections::HashMap;

    pub fn count_words(text: &str) -> HashMap<&str, i32> {
        let mut map = HashMap::new();
    
        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }
        
        map
    }