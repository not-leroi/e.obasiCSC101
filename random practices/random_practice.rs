use std::io;

fn main() {
 for _ in 0..50 {
 	let tuition_for_engineering:f32 = 1_000_000.00;
 	let tuition_for_medicine:f32 = 1_200_000.00;
 	let tuition_for_law:f32 = 800_000.00;
 	let tuition_for_computer_science:f32 = 900_000.00;
 		println!("Welcome to Pan-Atlantic University!");

	println!("What is your name?");
	let mut name = String::new();
	io::stdin().read_line(&mut name).expect("Failed to read input");
	println!("Hi there {}", name);

	println!("Please enter your date of birth (DD/MM/YYYY)");
	let mut dob = String::new();
	io::stdin().read_line(&mut dob).expect("Failed to read input");

	println!("How old are you {}?",name );
	let mut age =String::new();
	io::stdin().read_line(&mut age).expect("Failed to read input");
	let age:i32 = age.trim().parse().expect("Not a valid number");

	println!("Kindly enter your email address");
	let mut email = String::new();
	io::stdin().read_line(&mut email).expect("Failed to read input");

	println!("Enter your phone number");
	let mut contact = String::new();
	io::stdin().read_line(&mut contact).expect("Failed to read input");
	let contact:i32 = contact.trim().parse().expect("Not a valid number");

	println!("What course are you studying {}?", name );
	let mut course = String::new();
	io::stdin().read_line(&mut course).expect("Failed to read input");
	let course = course.trim().to_lowercase();

	println!("Kindly input your GPA");
	let mut gpa = String::new();
	io::stdin().read_line(&mut gpa).expect("Failed to read input");
	let gpa:f32= gpa.trim().parse().expect("Not a valid float");

	println!("{} which state do you come from?",name );
	let mut state_of_origin = String::new();
	io::stdin().read_line(&mut state_of_origin).expect("Failed to read input");
	let state_of_origin = state_of_origin.trim().to_lowercase();

	println!("What is your family income level");
	let mut income_level = String::new();
	io::stdin().read_line(&mut income_level).expect("Failed to read input");
	let income_level:i32= income_level.trim().parse().expect("Not a valid number");

	if course == "engineering" && age<25 && gpa>4.5 && state_of_origin == "lagos"{
		println!("You get a 50% scholarship.The amount to be paid is {} ", tuition_for_engineering * 0.5);
		println!("Name: {}, Date of Birth: {}, Age: {}, Email Address: {}, Phone Number: {}, Course: {}, GPA: {}, State of Origin : {}, Family Income Level: {}, Amount:{}" ,name, dob, age, email, contact, course, gpa, state_of_origin, income_level, tuition_for_engineering * 0.5 );
	}else if  course == "medicine" && age<28 && income_level<500_000 && state_of_origin == "enugu"{
		println!("You get a 70% scholarship! The amount you'll pay is N{}", tuition_for_medicine * 0.3);
		println!("Name: {}, Date of Birth: {}, Age: {}, Email Address: {}, Phone Number: {}, Course: {}, GPA: {}, State of Origin : {}, Family Income Level: {}, Amount:{}" ,name, dob, age, email, contact, course, gpa, state_of_origin, income_level, tuition_for_medicine * 0.3);
	}else if course == "law" && age<30 && gpa>4.0 && income_level<300_000{
		println!("You earn a scholarship of 40%. The tuition to be paid is N{}.", tuition_for_law * 0.6);
				println!("Name: {}, Date of Birth: {}, Age: {}, Email Address: {}, Phone Number: {}, Course: {}, GPA: {}, State of Origin : {}, Family Income Level: {}, Amount:{}" ,name, dob, age, email, contact, course, gpa, state_of_origin, income_level, tuition_for_law * 0.6);
	}else if course == "computer science" && age<23 && gpa>4.8 && state_of_origin == "kano"{
		println!("Congrats! You get a 30% scholarship. The amount to be paid is N{}", tuition_for_computer_science * 0.7);
						println!("Name: {}, Date of Birth: {}, Age: {}, Email Address: {}, Phone Number: {}, Course: {}, GPA: {}, State of Origin : {}, Family Income Level: {}, Amount:{}" ,name, dob, age, email, contact, course, gpa, state_of_origin, income_level, tuition_for_computer_science * 0.7);
	}else {
		println!("Sorry. You don't qulaify for a scholarship.");
	}
 }
}