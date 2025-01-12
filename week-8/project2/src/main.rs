use std::fs::File;
use std::io::{self, Write};

struct Student {
    name: String,
    matric_number: String,
    department: String,
    level: u32,
}

fn main() -> io::Result<()> {
    // Define the students using a vector
    let students = vec![
        Student {
            name: String::from("Oluchi Mordi"),
            matric_number: String::from("ACC1021111"),
            department: String::from("Accounting"),
            level: 300,
        },
        Student {
            name: String::from("Adams Aliyu"),
            matric_number: String::from("ECO10110101"),
            department: String::from("Economics"),
            level: 100,
        },
        Student {
            name: String::from("Shania Bolade"),
            matric_number: String::from("CSC1032882"),
            department: String::from("Computer"),
            level: 200,
        },
        Student {
            name: String::from("Adekunle Gold"),
            matric_number: String::from("EEE1010202"),
            department: String::from("Electrical"),
            level: 400,
        },
        Student {
            name: String::from("Blanca Edemoh"),
            matric_number: String::from("MEE1020201"),
            department: String::from("Mechanical"),
            level: 100,
        },
    ];

    // Display student details in the console
    println!("PAU SMIS Student Records:");
    println!("-----------------------------------------");
    println!(
        "{:<20} {:<15} {:<15} {:<5}",
        "Student Name", "Matric Number", "Department", "Level"
    );
    for student in &students {
        println!(
            "{:<20} {:<15} {:<15} {:<5}",
            student.name, student.matric_number, student.department, student.level
        );
    }

    // Save student details to a file
    let mut file = File::create("student_records.txt")?;
    writeln!(file, "PAU SMIS Student Records:")?;
    writeln!(file, "-----------------------------------------")?;
    writeln!(
        file,
        "{:<20} {:<15} {:<15} {:<5}",
        "Student Name", "Matric Number", "Department", "Level"
    )?;
    for student in &students {
        writeln!(
            file,
            "{:<20} {:<15} {:<15} {:<5}",
            student.name, student.matric_number, student.department, student.level
        )?;
    }

    println!("\nStudent records have been saved to 'student_records.txt'");
    Ok(())
}
