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

  s.truncate(3);
  println!("the first word is: {:?}", s);

  let s1 = String::from("hey, ");
  let s2 = String::from("you!");
  let s3 = s1 + &s2; //s1 moved
  println!("the first word is: {:?}", s3);
}
