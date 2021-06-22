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

pub fn print_person() {
    println!("traittest print_person");

    let tool = Tool{name:String::from("gai zhui")};

    let zj = Worker {
        username: String::from("zj"),
        ref_tool: &tool,
    };

    run(&zj);
    name(&zj);
}
