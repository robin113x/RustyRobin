use std::io;
fn main(){
	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("Fail");

	let mut parts: Vec<&str> = input.trim().split_whitespace().collect();
    let a: i32 = parts[0].parse().expect("Invalid input for A");
    let b: i32 = parts[1].parse().expect("Invalid input for B");
    let c: i32 = parts[2].parse().expect("Invalid input for C");
    let mut parts = vec![a,b,c];
    parts.sort();
    for x in parts.iter(){
        println!("{x}");
    }
    println!();
    println!("{a}");
    println!("{b}");
    println!("{c}");

    
}