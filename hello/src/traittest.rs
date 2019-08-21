trait Person {
    fn run(&self) {
        print!("Person run\n");
    }

    fn name(&self) -> String;
}

fn run<T: Person>(person: &T) {
    person.run();
}

fn name(person: &impl Person) {
    print!("name = {}\n", person.name());
}

struct Worker {
    username: String,
}

impl Person for Worker {
    fn run(&self) {
        print!("Worker run\n");
    }

    fn name(&self) -> String {
        self.username.clone()
    }
}

pub fn print_person() {
    println!("traittest print_person");

    let zj = Worker {
        username: String::from("zj"),
    };

    run(&zj);
    name(&zj);
}
