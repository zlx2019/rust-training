
fn main(){
    {
        let name: Box<&str> = Box::new("Zero");
        let length = name.len(); // name-deref
        println!("{}", name);
    } // name-drop
}