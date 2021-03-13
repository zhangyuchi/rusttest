fn first_word(s: &String) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[0..i];
    }
  }

  &s[..]
}

pub fn test() {
  let mut s = String::from("hello world");
  println!("the first word is: {:?}", s);

  {
    let word = first_word(&s);
    println!("the first word is: {:?}", word);
  }

  s.truncate(2);
  println!("the first word is: {:?}", s);
}
