fn maptest() {
  use std::collections::HashMap;
  let mut scores = HashMap::new();
  {
    let mut key = String::from("Blue");
    scores.insert(key, 10);
    key = String::from("Red");
    scores.insert(key, 50);
  }

  for (key, value) in &scores {
    println!("{}: {}", key, value);
  }

  let text = "hello world wonderful world";
  let mut map = HashMap::new();
  for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
  }
  for (key, value) in &map {
    println!("{}: {}", key, value);
  }
}

pub fn test() {
  maptest();
}
