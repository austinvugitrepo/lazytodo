use std::io;

fn main() {
    println!("Type 'exit' to exit the program");
 loop {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("can't read input");
       //removing newline character
    let new_input = input.trim();
        if new_input == "exit"{
           break;
       } else {
           println!("writing '{}' to todo list...", new_input);
        }
   
       

  }


}
