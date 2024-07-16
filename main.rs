use std::any::type_name;
use std::io;

fn type_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}


fn main(){
	let mut arr:[i32;5] = [1,2,3,4,5];
	println!("{:?}",arr);
	println!("{}",arr.len());
	write_arr(&mut arr);
	println!("main fun arr {:?}",arr);
	let x=34;
	println!("{}", type_of(&x));
	println!("{}",type_of(&arr));



}
fn write_arr( arr:&mut [i32;5]){
	let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed t read line");
    let no : i32= input.trim().parse().expect("Enter valid no");
    arr[0]=no;
	println!("write fun arr {:?}",arr);
} 