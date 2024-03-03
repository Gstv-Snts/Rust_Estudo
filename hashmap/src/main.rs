use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("a", 100);
    scores.insert("b", 30);
    scores.insert("c", 10);
    scores.insert("d", 80);
    println!("{:?}", scores.get("e").unwrap_or(&0));
    scores.entry("c").or_insert(30);
    scores.entry("e").or_insert(100);
    for (key, value) in &scores {
        println!("Key: {}, Value: {}", key, value);
    }
    let text = String::from("alo hello lol alo");
    let mut text_map = HashMap::new();
    for word in text.split_whitespace() {
        let count = text_map.entry(word).or_insert(0);
        *count += 1;
        println!("Word: {}, count: {}", word, count)
    }
    println!("{:?}", text_map);
}
