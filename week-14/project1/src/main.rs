use std::io::Read;
use std::io;

fn project_manager(){
    let mut file = std::fs::File::open("project_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}",contents);
}

fn administrator(){
    let mut file = std::fs::File::open("globacom_dbase.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}",contents);
}

fn employee(){
    let mut file = std::fs::File::open("staff_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}",contents);
}

fn customer(){
    let mut file = std::fs::File::open("customer_table_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}",contents);
}

fn vendor(){
    let mut file = std::fs::File::open("dataplan_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}",contents);
}

fn main(){
    println!("Hello there. Are you an administrator, a project manager, a customer, a vendor or an employee?");
    let mut response = String::new();
    io::stdin().read_line(&mut response).expect("Failed to read input");
    let response = response.trim().to_lowercase();

    if response == "administrator"{
        administrator();
    }else if response == "project manager"{
        project_manager();
    }else if response == "employee"{
        employee();
    }else if response == "customer"{
        customer();
    }else if response == "vendor"{
        vendor();
    }
}