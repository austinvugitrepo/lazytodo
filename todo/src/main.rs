use std::io;
use std::io::Write;
use std::fs::File;

fn main() -> std::io::Result<()> {
    File::create_new("lazytodo.txt")?; // make todo file 
    println!("Type 'exit' to exit the program");
 loop {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("can't read input");
       //removing newline character
    let mut new_input = input.trim();
        if new_input == "exit"{
           break Ok(());
       } else {
           println!("writing '{}' to todo list...", new_input);
           let _ = iowrite(&mut new_input); //writes to lazytodo.txt
           println!("sucessfully added '{}' to todo list...", new_input);
        }
   
       

  }

}

fn iowrite(inputstr: &mut &str) -> std::io::Result<()> {
    let mut fappend = File::options().append(true).open("lazytodo.txt")?; //appending to file 
    writeln!(&mut fappend, "{}", inputstr)?; // write to lazytodo File 
    Ok(())

}
