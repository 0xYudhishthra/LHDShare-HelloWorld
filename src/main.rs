fn main(){
    println!("Hello, World!"); //Prints Hello, World!
    let mut line = String::new();
    println!("Enter your name :"); //Reads STDIN from console
    let _ = std::io::stdin().read_line(&mut line).unwrap();
    println!("Hello , {}", line); //Prints STDOUT stream
 }
