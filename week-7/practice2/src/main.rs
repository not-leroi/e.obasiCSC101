fn main() {
    let v = vec!['C','O','M','P','U','T','E','R'];
    let mut input1 =  String::new();

    println!("Enter an index value btw (0-7)");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");

    //index is the non megative value
    let index:usize = input1.trim().parse().expect("Failed to read input");

    //getting value at given index value
    let ch: char = v[index];
    print!("{} is the character for index [{}]\n",ch,index );
}