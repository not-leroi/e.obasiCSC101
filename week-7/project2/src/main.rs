use std::io;
fn main() {
    let mut candidates: Vec<(String, u32, u32)> = Vec::new();

    println!("How many developers were interviewed?");
    let mut num_of_candidates = String::new();
    io::stdin().read_line(&mut num_of_candidates).expect("Failed to read input");
    let num_of_candidates:u32 = num_of_candidates.trim().parse().expect("Input was not valid!");

    for _n in 0..num_of_candidates{
    println!("What is the name of the interview candidate?");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");

    println!("How old is {}?",name);
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Failed to read input");
    let age:u32 = age.trim().parse().expect("Input is not valid!");

    println!("How many years of work experience does {} have?",name );
    let mut work_experience = String::new();
    io::stdin().read_line(&mut work_experience).expect("Failed to read input");
    let work_experience:u32 = work_experience.trim().parse().expect("Input is not valid!");
    candidates.push((name, age, work_experience));
    }

    let mut most_experienced = String::new();
    let mut max_experience = 0;

    for c in candidates{
        let name = c.0;
        let work_experience = c.2;

        if work_experience > max_experience{
            max_experience = work_experience;
            most_experienced = name;
        }
    }
    println!("The most experienced developer is {} with {} years of work experience",most_experienced,max_experience);
}