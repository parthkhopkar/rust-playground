#[derive(Debug)]
enum Coin {
	Penny,
	Nickel,
	Dime,
	Quarter,
}

fn coin_value(coin: &Coin) -> u8 {
	match coin {
		Coin::Penny => 1,
		Coin::Nickel => 5,
		Coin::Dime => 10,
		Coin::Quarter => 25,
	}
}

fn main() {
    println!("Hello, world!");
    let dime = Coin::Dime;
    println!("{:?}", coin_value(&dime));
    println!("{:?}", dime);
}
