use std::io;
fn main(){
	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("Fail");

	let parts: Vec<&str> = input.trim().split_whitespace().collect();
    let a: i32 = parts[0].parse().expect("Invalid input for A");
    //let b: i32 = parts[1].parse().expect("Invalid input for B");

    for i in 1..13{
    	println!("{} * {} = {}",a,i,a*i)
    }

}