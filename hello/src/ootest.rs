
struct Rectangle {
    width : u32,
    height : u32,
}

struct Circle {
    x : u32,
    y : u32,
    radius : u32,
}

use std::any::Any;
trait  IShape : Any + 'static {
    fn area(&self) -> f32;
    fn to_string(&self) -> String;
    fn as_any(&self) -> &dyn Any;
}

impl IShape  for Rectangle {
    fn area(&self) -> f32 { (self.height * self.width) as f32 }
    fn to_string(&self) ->String {
        format!("Rectangle -> width={} height={} area={}",
                self.width, self.height, self.area())
    }
    fn as_any(&self) -> &dyn Any { self }
}

use std::f64::consts::PI;
impl IShape  for Circle  {
    fn area(&self) -> f32 { (self.radius * self.radius) as f32 * PI as f32}
    fn to_string(&self) -> String {
        format!("Circle -> x={}, y={}, area={}",
                self.x, self.y, self.area())
    }
    fn as_any(&self) -> &dyn Any {self}
}

pub fn test() {
    test_common();
    test_downcast();
}

fn test_common() {
    use std::vec::Vec;

    let rect = Box::new( Rectangle { width: 4, height: 6});
    let circle = Box::new( Circle { x: 0, y:0, radius: 5});
    let mut v: Vec<Box<dyn IShape>> = Vec::new();
    v.push(rect);
    v.push(circle);

    for i in v.iter() {
        println!("area={}", i.area() );
        println!("{}", i.to_string() );
    }
}

fn test_downcast() {
    let mut v : Vec<Box<dyn IShape>> = Vec::new();
    let rect = Box::new( Rectangle { width: 4, height: 6});
    let circle = Box::new( Circle { x: 0, y:0, radius: 5});
    v.push(rect);
    v.push(circle);
    for i in v.iter() {
        if let Some(s) = i.as_any().downcast_ref::<Rectangle>() { //note:RTTI
            println!("downcast - Rectangle w={}, h={}", s.width, s.height);
        }else if let Some(s) = i.as_any().downcast_ref::<Circle>() {
            println!("downcast - Circle x={}, y={}, r={}", s.x, s.y, s.radius);
        }else{
            println!("invaild type");
        }
    }
}
