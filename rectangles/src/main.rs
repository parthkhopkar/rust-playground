#[derive(Debug)]  // This annotation enables prettyprinting of the struct
struct Rectangle {
	width: u32,
	height: u32,
}

fn main() {
	let rect1 = Rectangle{
		width: 40,
		height: 50,
	};

    println!("The area of the rectangle {:?} is {:?} units", rect1, area(&rect1));

}


// A function which takes in a Rectangle struct and returns its area
fn area(rectangle: &Rectangle) -> u32 {
	rectangle.width * rectangle.height
}