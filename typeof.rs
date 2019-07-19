// this is the starting of an idea for a Rust function that returns the current type. I will probably use it for an upcoming open source project.

use std::io;

fn typeid<T: std::any::Any>(_: &T) {
    println!("{:?}", std::any::TypeId::of::<T>() == std::any::TypeId::of::<String>()); // refactor this using a match block
}

fn main() {
	println!("Guess the number!");
	println!("Please input your guess!");
	let mut guess = String::new();
	io::stdin().read_line(&mut guess).expect("Failed to read line");
	println!("You guessed: {}", guess);
	typeid(&guess)
}
