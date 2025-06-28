use std::collections::HashMap;

pub fn counting_words(words: &str) -> HashMap<String, u32> {
    let mut s = String::new();
    for c in words.chars() {
        if c.is_alphanumeric() || c.is_whitespace() || c == '\'' {
            s.push(c)
        }
    }
    let s = s.replace("' ", " ").replace(" '", " ");
    let sp = s.split_whitespace();
    let mut mymap = HashMap::new();
    for word in sp {
        *mymap.entry(word.to_lowercase()).or_insert(0) += 1
    }
    mymap
}
