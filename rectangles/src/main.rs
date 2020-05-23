#[derive(Debug)]  // This annotation enables prettyprinting of the struct
struct Rectangle {
	width: u32,
	height: u32,
}

// A function which takes in a Rectangle struct and returns its area
fn area(rectangle: &Rectangle) -> u32 {
	rectangle.width * rectangle.height
}

#[derive(Debug)]
struct Circle {
	radius: f64,
}

// Methods for the Circle struct
impl Circle {
	fn area(&self) -> f64{
		let PI = std::f64::consts::PI;
		PI * self.radius * self.radius
	}
}

fn main() {
	let rect1 = Rectangle{
		width: 40,
		height: 50,
	};

    println!("The area of the rectangle {:?} is {:?} square units", rect1, area(&rect1));

    let circ1 = Circle{
    	radius: 10.0,
    };

    println!("The area of the circle {:?} is {:?} square units", circ1, circ1.area());

}

