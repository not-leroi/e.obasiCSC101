use std::io;

fn main() {
    println!("Hello there, what is your name?");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");
    println!("Hi {}, what shape do you wish to find the area of?",name );
    println!("T = Area of Trapezium");
    println!("R = Area of a rhombus");
    println!("P = Area of parallelogram");
    println!("Cu = Area of a cube");
    println!("C = Cylinder");
    println!("Kindly input the letters before the '=' sign");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    let choice = choice.trim().to_lowercase();
    if choice == "t" {
        trapezium();
    }else if choice == "r"{
        rhombus();
    }else if choice == "p"{
        parallelogram();
    }else if choice == "cu"{
        cube();
    }else if choice == "c"{
        cylinder();
    }else {
        println!("No option chosen");
    }
}

fn trapezium() {
    println!("Input the height");
    let mut height = String::new();
    io::stdin().read_line(&mut height).expect("Failed to read input");
    let height:i32 = height.trim().parse().expect("Not a valid integer");

    println!(" Input your first base");
    let mut base1 = String::new();
    io::stdin().read_line(&mut base1).expect("Failed to read input");
    let base1:i32 = base1.trim().parse().expect("Not a valid integer");

    println!("Now your second base");
    let mut base2 = String::new();
    io::stdin().read_line(&mut base2).expect("Failes to read input");
    let base2:i32 = base2.trim().parse().expect("Not a valid integer");

    let trapezium_formula = height/2 * (base1+base2);
    println!("The area of your trapezium is :{}",trapezium_formula);
}

fn rhombus() {
    println!("Input the first diagonal");
    let mut diagonal1 = String::new();
    io::stdin().read_line(&mut diagonal1).expect("Failed to read input");
    let diagonal1:i32 =diagonal1.trim().parse().expect("Not a valid integer");

    println!("Input the second diagonal");
    let mut diagonal2 = String::new();
    io::stdin().read_line(&mut diagonal2).expect("Failed to read input");
    let diagonal2:i32 =diagonal2.trim().parse().expect("Not a valid integer");

    let rhombus_formula : f32= (1/2 * diagonal1 * diagonal2) as f32;
    println!("The area of your rhombus is :{}",rhombus_formula);   
}

fn parallelogram() {
    println!("Input the base");
    let mut base = String::new();
    io::stdin().read_line(&mut base).expect("Failed to read input");
    let base:i32 = base.trim().parse().expect("Not a valid integer");
    println!("Now input the altitude");
    let mut altitude = String::new();
    io::stdin().read_line(&mut altitude).expect("Failed to read input");
    let altitude:i32 = altitude.trim().parse().expect("Not a valid integer");

    let parallelogram_formula = base * altitude;
    println!("The area of your parallelogram is:{}",parallelogram_formula);
}

fn cube() {
    println!("What is the length of the side?");
    let mut side = String::new();
    io::stdin().read_line(&mut side).expect("Failed to read input");
    let side:i32 = side.trim().parse().expect("Not a valid integer");

    let cube_formula = 6 * side.pow(2);
    println!("The area of your cube is:{}",cube_formula);
}

fn cylinder() {
    println!("What's the radius?");
    let mut radius = String::new();
    io::stdin().read_line(&mut radius).expect("Failed to read input");
    let radius:i32 = radius.trim().parse().expect("Not a valid integer");
    println!("What is the height");
    let mut height = String::new();
    io::stdin().read_line(&mut height).expect("Failed to read input");
    let height:i32 = height.trim().parse().expect("Not a valid integer");
    let cylinder_formula :f32 = (22/7 * radius.pow(2) * height) as f32;
    println!("The volume of your cylinder: {}",cylinder_formula);
}