pub mod args;
pub mod plist;
pub mod constants;
pub mod commands;
pub mod util;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = args::get();

    match args.run() {
        Ok(_) => println!("Success"),
        Err(e) => {
            println!("Error: {}", e);
            Err(e)?
        },
    };

    // build the application
    println!("Hello, world!");

    Ok(())
}
