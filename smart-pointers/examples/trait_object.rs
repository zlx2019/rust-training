trait Animal {
    fn call(&self);
}

struct Cat {
    name: String,
}
impl Cat {
    fn new(name: String) -> Cat {
        Cat { name }
    }
}
impl Animal for Cat {
    fn call(&self) {
        println!("{}: 喵喵喵!", self.name);
    }
}

struct Dog {
    name: String,
}
impl Dog {
    fn new(name: String) -> Dog {
        Dog { name }
    }
}
impl Animal for Dog {
    fn call(&self) {
        println!("{}: 汪汪汪!", self.name);
    }
}

fn main(){
    let mut animal: Box<dyn Animal> = Box::new(Cat::new("Tom".to_string()));
    animal.call(); // Tom: 喵喵喵!

    animal = Box::new(Dog::new(String::from("Spike")));
    animal.call(); // Spike: 汪汪汪!
}
