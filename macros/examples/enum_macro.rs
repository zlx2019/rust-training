use macros::EnumFrom;


#[derive(Debug, EnumFrom)]
enum Direction {
    Up(DirectionUp),
    Down
}
#[derive(Debug)]
struct DirectionUp {
    speed: u32
}

fn main(){
    let up = Direction::Up(DirectionUp { speed: 50 });
    println!("{:?}", up);
}