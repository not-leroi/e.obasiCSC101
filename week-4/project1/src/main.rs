use std::io;

fn main() {
		println!("Input a");
		let mut input1 = String::new();
		io::stdin().read_line(&mut input1).expect("Failed to read input");
		let a:f32 = input1.trim().parse().expect("Not a valid number");

		println!("Input b");
		let mut input2 = String::new();
		io::stdin().read_line(&mut input2).expect("Failed to read input");
		let b:f32 = input2.trim().parse().expect("Not a valid number");

		println!("Input c");
		let mut input3 = String::new();
		io::stdin().read_line(&mut input3).expect("Failed to read input");
		let c:f32 = input3.trim().parse().expect("Not a valid number");

			let discriminant:f32 = b.powf(2.0) -4.0*a*c.sqrt();

			if discriminant >0.0 {
				println!("There are 2 real roots");
			} else if discriminant ==0.0 {
				println!("There is one real root");
			} else if discriminant <0.0 {
				println!("There is no real root");
			}
}
