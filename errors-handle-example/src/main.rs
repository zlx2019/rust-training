use fs::list_files;
use types::Result;

mod types;
mod fs;


fn main() -> Result<()>{
    println!("Hello, world!");
    let files = list_files(".")?;
    println!("{files:#?}");
    Ok(())
}


