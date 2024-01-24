use std::{env, fs::File, io::Read, process::exit};

fn main() {
    // First argument is the name of the binary
    let a: Vec<String> = env::args().collect();
    let mut opcode = [0; 4]; // opcode is always 4 bytes

    if a.len() < 2 {
        println!("ERROR: You need to pass filename for the ROM");
        exit(1);
    }
    let filename = &a[1];
    println!("You pass {filename}");

    let mut f = File::open(filename).unwrap();

    loop {
        // Read 4 bytes by 4 bytes
        match f.read_exact(&mut opcode) {
            Ok(()) => println!("{} {} {} {}", opcode[0], opcode[1], opcode[2], opcode[3]),
            Err(_) => break,
        }
    }
}
