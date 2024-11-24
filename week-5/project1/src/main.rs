use std::io;
fn main() {
    println!("Welcome to The Kings Restaurant. What would you like to order");
    println!("P = Pounded Yam/Edikainko Soup - N3,200");
    println!("F = Fried Rice & Chicken - N3,000");
    println!("A = Amala & Ewudu Soup - N2,500");
    println!("E = Eba & Egusi Soup - N2,000");
    println!("W = White Rice & Stew - N2,500");

    let mut order = String::new();
    println!("What food item do you choose? Choose from P, F, A, E or W");
    io::stdin().read_line(&mut order).expect("Failed to read input");

    let mut quantity = String::new();
    println!("How many portions do you want?");
    io::stdin().read_line(&mut quantity).expect("Failed to read input");
    let quantity:i32 = quantity.trim().parse().expect("Failed to read input");

    let price: i32 = if order.trim() == "P" {
        3200 
    }else if order.trim() == "F"{
         3000 
    }else if order.trim() == "A" {
        2500 
    }else if order.trim() == "E"{
         2000 
    }else if order.trim() == "W"{
         2500
    }else {
        println!("Not part of the menu");
        return;
    };

    let total_price = price * quantity;
    let final_total = if total_price>10000{
        total_price as f32 * 0.95
    }else {
        total_price as f32
    };
    println!("Order Summary:");
    println!("Your order is {}", order);
    if total_price >10000{
        println!("You get a discount of 5%! Your bill is N{}",total_price as f32 * 0.95 );
    }else {
        println!("Your bill is N{}",final_total );
    }
    println!("Thank you patronizing us. Do come again!");
}