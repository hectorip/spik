use std::io::Read;
use std::io::Write;

fn main() {
    // take file input from arguments
    let args: Vec<String> = std::env::args().collect();
    let file = &args[1];
    print!("File: {}", file);

    // read file
    let mut file = std::fs::File::open(file).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("\n{}", contents.len());
    // parse and convert to an AST
    // code generation
    // output to an XML
    // writr to a file
    let mut output = std::fs::File::create("output.xml").unwrap();
    output.write_all(contents.as_bytes()).unwrap();

}