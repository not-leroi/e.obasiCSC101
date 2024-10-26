fn main() {
	//Sales amounts
	let amounts = [450_000.0, 1_500_000.0, 750_000.0, 2_850_000.0, 250_000.0];

	//Calculate sum
	let sum:f64 = amounts.iter().sum();

	//Calculate average
	let average = sum/amounts.len() as f64;
	println!("Sum : {}", sum);
	println!("Average: {}", average);
	
}