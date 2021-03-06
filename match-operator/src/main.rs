#[derive(Debug)]
enum Coin {
	Penny,
	Nickel,
	Dime,
	Quarter(String),
}

fn coin_value(coin: &Coin) -> u8 {
	match coin {
		Coin::Penny => 1,
		Coin::Nickel => 5,
		Coin::Dime => 10,
		Coin::Quarter(state) => {
			println!("You have a Quarter from the US state of {:?}", state);
			25
		},
	}
}

fn detect(x: Option<i32>) -> bool {
	match x {
		Some(x) => true,
		None => false,
	}
}

fn coin_value_with_placeholder(coin: &Coin) -> u8 {
	match coin {
		Coin::Penny => 1,
		Coin::Nickel => 5,
		_ => 0,
	}
}

fn main() {
    println!("Hello, world!");
    let dime = Coin::Dime;
    println!("{:?}", coin_value(&dime));
    println!("{:?}", dime);

    let quarter = Coin::Quarter(String::from("Arizona"));
    println!("{:?}", coin_value(&quarter));
    println!("{:?}", quarter);

    // match with Option<T> using detect function
    let x = Some(10);
    println!("Variable x is a number?: {:?}", detect(x));
    let x: Option<i32> = None;
    println!("Variable x is a number?: {:?}", detect(x));

    // Use placeholder value to only match Penny and Nickel
    let penny = Coin::Penny;
    println!("{:?}", coin_value_with_placeholder(&penny));
    println!("{:?}", penny);

    // Using if let to just match one arm
    let state = String::from("Arizona");
    if let Coin::Quarter(state) = quarter{
    	println!("You have a quarter from the state of {:?}", state);
    } else {
    	println!("Your have a coin other than a Quarter");
    }

}
