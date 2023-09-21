use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    {
        let mut file = File::create("empty_file.mixexe")?;
        // Write a slice of bytes to the file
        file.write_all(&[109, 105, 120, 101, 120, 101])?;
    }

    {
        let mut file = File::open("empty_file.mixexe")?;
        // read the same file back into a Vec of bytes
        let mut buffer = Vec::<u8>::new();
        file.read_to_end(&mut buffer)?;
        println!("{:?}", buffer);
    }

    Ok(())
}
