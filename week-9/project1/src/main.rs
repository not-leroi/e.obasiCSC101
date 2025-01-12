struct Laptops{
    brand:String,
    no_avalailable:u32,
    price_for_one:u32,
    no_purchased:u32
}

impl Laptops{
    fn price_for_a_brand(&self)-> u32{
        self.no_purchased*self.price_for_one
    }
}

fn main(){
    let lap1 = Laptops{
        brand:String::from("HP"),
        no_avalailable:10,
        price_for_one:650_000,
        no_purchased:3,
    };
    let lap2 = Laptops{
        brand:String::from("IBM"),
        no_avalailable:6,
        price_for_one:755_000,
        no_purchased:3,
    };
    let lap3 = Laptops{
        brand:String::from("Toshiba"),
        no_avalailable:10,
        price_for_one:550_000,
        no_purchased:3,
    };
    let lap4 = Laptops{
        brand:String::from("Dell"),
        no_avalailable:4,
        price_for_one:850_000,
        no_purchased:3,
    };
    println!("Your purchased 3 laptops.");
    println!("The total cost is N{}",lap1.price_for_a_brand()+lap2.price_for_a_brand()+lap3.price_for_a_brand()+lap4.price_for_a_brand());
}