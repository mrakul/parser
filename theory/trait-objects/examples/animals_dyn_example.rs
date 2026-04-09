trait Mammal {
    fn walk(&self);
    fn run(&self);
}

#[derive(Clone)]
struct Cat {
    meow_factor: u8,
    purr_factor: u8
}

impl Mammal for Cat {
    fn walk(&self) {
        println!("Cat::walk");
    }
    fn run(&self) {
        println!("Cat::run")
    }
}

struct Animals {
    subjects: Vec<Box<dyn Mammal>>,
}

fn main() {

}