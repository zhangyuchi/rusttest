//add lifetime trait
trait Person<'life> {
    fn run(&self) {
        print!("Person run\n");
    }

    fn name(&self) -> String;

    fn set_tool(&mut self, tool: &'life Tool);
}

fn run<'life, T: 'life + Person<'life>>(person: &'life T) {
    person.run();
}

fn name<'life>(person: &'life impl Person<'life>) {
    print!("name = {}\n", person.name());
}

struct Tool {
    name: String,
}

struct Worker<'life> {
    username: String,
    ref_tool : &'life Tool
}

impl<'life> Person<'life> for Worker<'life> {
    fn run(&self) {
        print!("Worker run\n");
    }

    fn name(&self) -> String {
        self.username.clone()
    }

    fn set_tool(&mut self, tool : &'life Tool) {
        self.ref_tool = tool
    }
}

pub fn test() {
    print_person();
    print_employee();
}

fn print_person() {
    println!("traittest print_person");

    let tool = Tool{name:String::from("gai zhui")};

    let zj = Worker {
        username: String::from("zj"),
        ref_tool: &tool,
    };

    run(&zj);
    name(&zj);
}

use std::cmp::{Ord, PartialOrd, PartialEq, Ordering};
#[derive(Debug)]
struct Employee {
    name : String,
    salary : i32,
}
impl Ord for Employee {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.salary.cmp(&rhs.salary)
    }
}
impl PartialOrd for Employee {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}
impl Eq for Employee {
}
impl PartialEq for Employee {
    fn eq(&self, rhs: &Self) -> bool {
        self.salary == rhs.salary
    }
}

fn print_employee() {
    let mut v = vec![
        Employee {name : String::from("Bob"),     salary: 2048},
        Employee {name : String::from("Alice"),   salary: 3208},
        Employee {name : String::from("Tom"),     salary: 2359},
        Employee {name : String::from("Jack"),    salary: 4865},
        Employee {name : String::from("Marray"),  salary: 3743},
        Employee {name : String::from("Hao"),     salary: 2964},
        Employee {name : String::from("Chen"),    salary: 4197},
    ];

//用for-loop找出薪水最多的人
    let mut e = &v[0];
    for i in 0..v.len() {
        if *e < v[i] {
            e = &v[i];
        }
    }
    println!("max = {:?}", e);

//使用标准的方法
    println!("min = {:?}", v.iter().min().unwrap());
    println!("max = {:?}", v.iter().max().unwrap());

//使用标准的排序方法
    v.sort();
    println!("{:?}", v);
}
