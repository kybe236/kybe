// #[derive(Debug)]

use std::io::{self, Write};

struct User {
  firt_name_s: String,
  last_name_s: String
}


fn main(){

  let first_name = input("First Name >> ");
  let last_name = input("last Name >> ");

  let standart_user = User{
    firt_name_s: first_name,
    last_name_s: last_name
  };


  println!("\nHello {} {}\n", standart_user.firt_name_s, standart_user.last_name_s);
  println!("Exiting...");
}



fn input(printy: &str) -> String {
  print!("{}",printy);
  let _ = io::stdout().flush().unwrap();

  let mut x = String::new();

  match io::stdin().read_line(&mut x) {
    Ok(_) => (),
    Err(err) => println!("Could not parse input: {}", err)
  }

  trim_newline(&mut x);
  return x.to_string();
}





fn trim_newline(s: &mut String) {
  if s.ends_with('\n') || s.ends_with('\r') {
      s.pop();
      if s.ends_with('\r') {
          s.pop();
      }
  }
}
