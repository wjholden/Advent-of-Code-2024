use std::fs::File;
use std::io::Write;

fn main() {

    for i in 1..=25 {
        let s = format!("src/bin/day{:02}.rs", i);
        match File::create(&s) {
            Ok(mut f) => {
                println!("Created {}", s);
                f.write_all(b"fn main() {}").expect("wrote something")
            },
            Err(e) => println!("Error {}", e)
        };
    }

}