fn main(){
    println!("Hello, World");
    let mut line = String::new();
    println!("Enter your name :");
    let _ = std::io::stdin().read_line(&mut line).unwrap();
    println!("Hello , {}", line);
 }