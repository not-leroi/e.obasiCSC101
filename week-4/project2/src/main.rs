use std::io;

fn main() {
    println!("How old are you?");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let age:i32 = input1.trim().parse().expect("Not a valid number");


    println!("Are you experienced or inexperienced?");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let experience = input2.trim().to_lowercase();

    if experience =="experienced" && age >= 40{
        println!("Your incentive is N1,560,000");
    } else if experience == "experienced" && age >=30 && age <=40{
        println!("Your incentive is N1,480,000");
    } else if experience == "experienced" && age <=28{
        println!("Your incentive is N1,300,000");
    } else if experience == "inexperienced"{
        println!("Your incentive is N100,000");
    }
}