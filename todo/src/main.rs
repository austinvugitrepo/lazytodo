use std::io;

fn main() {
    println!("Type 'exit' to exit the program");
    escape_program(); // loop that ends with 'exit' typed by user 

}
fn escape_program() {
    loop {
      let mut input = String::new();
       io::stdin().read_line(&mut input).expect("can't read input");

       //removing newline character
       let escape = input.trim();

       if escape == "exit" {
            break;
        }
    }

}
