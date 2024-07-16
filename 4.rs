use std::io;
fn main(){
	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("Fail");

	let parts: Vec<&str> = input.trim().split_whitespace().collect();
    let no: i32 = parts[0].parse().expect("Invalid input for A");

    for i in 1..no+1{
    	if no%i==0{
    		println!("{i}");
    	}
    }

}
//https://codeforces.com/group/MWSDmqGsZm/contest/219432/problem/K