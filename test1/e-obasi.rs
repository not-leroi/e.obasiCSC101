use std::io;
fn main() {
	let _amount_for_alzheimer:f32 = 1_200_000.0;
	let _amount_for_arrhythmia:f32 = 550_000.0;
	let _amount_for_chronic_kidney_disease:f32 = 1_500_000.0;
	let _amount_for_diabetes:f32 = 800_000.0;
	let _amount_for_arthritis:f32 = 450_000.0;
	for _ in 0..100 {
			println!("What is your name?");
	let mut name = String::new();
	io::stdin().read_line(&mut name).expect("Failed to read input");
	println!("Hello {}",name);

	println!("What year were you born in?");
	let mut dob = String::new();
	io::stdin().read_line(&mut dob).expect("Failed to read input");
	let dob:i32 = dob.trim().parse().expect("Input is not an integer");
	let age:i32 = 2024 - dob;

	println!("Please enter your email address");
	let mut email = String::new();
	io::stdin().read_line(&mut email).expect("Failed to read input");
	println!("Your email address is {}", email);

	println!("Enter your phone number");
	let mut number = String::new();
	io::stdin().read_line(&mut number).expect("Failed to read input");
	let number:i32 = number.trim().parse().expect("input is not an integer");

	println!("How may siblings do you have?");
	let mut siblings = String::new();
	io::stdin().read_line(&mut siblings).expect("Failed to read input");
	let siblings:i32 = siblings.trim().parse().expect("Input is not an integer");

	println!("How many children do you have?");
	let mut children = String::new();
	io::stdin().read_line(&mut children).expect("Failed to read input");
	let children:i32 = children.trim().parse().expect("Input is not an integer");

	println!("What is your medical diagnosis?");
	let mut diagnosis = String::new();
	io::stdin().read_line(&mut diagnosis).expect("Failed to read input");
	let diagnosis =diagnosis.trim().to_lowercase();

	println!("What village do you reside in?");
	let mut village_of_residence = String::new();
	io::stdin().read_line(&mut village_of_residence).expect("Failed to read input");
	let village_of_residence =village_of_residence.trim().to_lowercase();

	if diagnosis == "alzheimer"{
		println!("The amount to be paid is N{}", _amount_for_alzheimer);
		println!("{}, {}, {}, {}, {}, {}, {}, {}",name, dob, email, number, siblings, children, diagnosis, village_of_residence);
	} else if diagnosis =="alzheimer" && age>50 && children>4 && village_of_residence =="akpabom"{
		println!("You receive a 20% discount. Your amount to be paid is N{}",_amount_for_alzheimer * 0.8 );
	}else {
		
	}
	
	if diagnosis == "arrythmia"{
		println!("The amout to be paid is {}",_amount_for_arrhythmia );
		println!("{}, {}, {}, {}, {}, {}, {}, {}",name, dob, email, number, siblings, children, diagnosis, village_of_residence);
	}else if diagnosis =="arrythmia" && age == 30 && siblings > 4 && village_of_residence =="ngbauji"{
		println!("You receive a 5% discount. Your amount to be paid is N{}",_amount_for_arrhythmia * 0.95);
		println!("{}, {}, {}, {}, {}, {}, {}, {}",name, dob, email, number, siblings, children, diagnosis, village_of_residence);
	}else{}

	if diagnosis == "chronic kidney disease"{
		println!("The amount to be paid is N{}",_amount_for_chronic_kidney_disease);
		println!("{}, {}, {}, {}, {}, {}, {}, {}",name, dob, email, number, siblings, children, diagnosis, village_of_residence);
	}else if diagnosis == "chronic kidney disease" && age >40 && siblings >3 && children >3 && village_of_residence == "atabrikang"{
		println!("You receive a 15% discount. Your amount to be paid is N{}",_amount_for_chronic_kidney_disease * 0.85);
		println!("{}, {}, {}, {}, {}, {}, {}, {}",name, dob, email, number, siblings, children, diagnosis, village_of_residence);
	}else {
	}
	if diagnosis == "diabetes"{
		println!("The amount to be paid is N{}",_amount_for_diabetes );
		println!("{}, {}, {}, {}, {}, {}, {}, {}",name, dob, email, number, siblings, children, diagnosis, village_of_residence);
	}else if diagnosis == "diabetes" && age >28 && age<45 && children>=2 && children <=4 && village_of_residence == "okorobilom"{
		println!("You receive a 10% discount. The amount to be paid is N{}",_amount_for_diabetes * 0.9 );
		println!("{}, {}, {}, {}, {}, {}, {}, {}",name, dob, email, number, siblings, children, diagnosis, village_of_residence);
	}else {
	}
	if diagnosis == "arthritis"{
		println!("The amount to be paid is N{}",_amount_for_arthritis);
		println!("{}, {}, {}, {}, {}, {}, {}, {}",name, dob, email, number, siblings, children, diagnosis, village_of_residence);
	}else if diagnosis == "arthritis" && age >58 && siblings>5 && children>5 && village_of_residence == "emeremen"{
		println!("You receive a 10% discount. The amount to be paid is N{}",_amount_for_arthritis * 0.9 );
		println!("{}, {}, {}, {}, {}, {}, {}, {}",name, dob, email, number, siblings, children, diagnosis, village_of_residence);
	}else {
	}
	}
}