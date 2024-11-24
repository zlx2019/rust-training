// 使用 app_lib package中的 user crate
use app_lib::user;

fn main() {
    let _user = user::entity::UserEntity{age: 19};
    println!("Hello, world!");
}
