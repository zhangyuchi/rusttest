fn largest<T: PartialOrd>(list: &[T]) -> &T {
  //T是PartialOrd类型，才可以比较大小
  let mut max = &list[0]; //max是一个可变引用

  for item in list.iter() {
    //item是一个不可变引用
    if item > max {
      max = item;
    }
  }
  max
}

pub fn largest_run() {
  println!("template largest_run");

  let number_list = vec![14000, 50, 25, 100, 65];

  let result = largest(&number_list); //隐含指定模板参数
  println!("The largest number is {}", result);

  let char_list = vec!['y', 'm', 'a', 'q'];

  let result = largest::<char>(&char_list); //严格指定模板参数
  println!("The largest char is {}", result);
}

trait Stream {
  fn get_info(&self) -> String;
}

struct Console;
struct FileStream;

impl Console {
  fn gettype(&self) -> String {
    return String::from("console");
  }
}

impl FileStream {
  fn gettype(&self) -> String {
    return String::from("filesystem");
  }
}

impl Stream for Console {
  fn get_info(&self) -> String {
    return String::from("info:") + self.gettype().as_str();
  }
}

impl Stream for FileStream {
  fn get_info(&self) -> String {
    return String::from("info:") + self.gettype().as_str();
  }
}

//动多态，运行时派发
fn dynamic_dispatch(stream: &mut dyn Stream) {
  println!("dynamic_dispatch: {}", stream.get_info())
  // do something
}

//静多态，编译器派发
fn static_dispatch<T: Stream>(stream: &mut T) {
  println!("static_dispatch: {}", stream.get_info())
  // do something
}
