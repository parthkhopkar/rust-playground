// An enumeration of the type of IP addresses
#[derive(Debug)]
enum IpAddrKind {
	V4,
	V6,
}

// An enumeration with data types specified
#[derive(Debug)]
enum IpAddrKindWithDataType {
	V4(u8, u8, u8, u8),
	V6(String),
}

// A struct to which defines the kind of IP Address data
#[derive(Debug)]
struct IpAddr{
	kind: IpAddrKind,
	address: String,
}

fn main() {
    println!("Hello, world!");

    // Define enums of type V4 and V6
    let ipv4 = IpAddrKind::V4;
    let ipv6 = IpAddrKind::V6;

    // Define structs of type IpAddr
    let home_ip = IpAddr{
    	kind: IpAddrKind::V4,
    	address: String::from("192.168.1.1"),
    };

    let office_ip = IpAddr{
    	kind: IpAddrKind::V6,
    	address: String::from("2607:f0d0:1002:51::4")
    };

    println!("Home IP: {:?}, Office IP: {:?}", home_ip, office_ip);

    let ipv4_type = IpAddrKindWithDataType::V4(128,0,1,1);
    let ipv6_type = IpAddrKindWithDataType::V6(String::from("2607:f0d0:1002:51::4"));

    println!("IPv4: {:?} IPV6: {:?}", ipv4_type, ipv6_type);

    // --> The Rust library also has an implementation of IP address types that we can use

    // Rust does not explicitly have a null type but it has an enum called Option that is part of the prelude(does not need to be brought into scope)
    /*
    enum Option<T> {
		Some(T),
		None,
    }
    */
    // Rust does not have a null type but the None in Option captures the concept of not having a value or having a value that is not valid.
    // Using Option is a way of telling yourself that the value might be null. Only when Option is used you need to worry having a null value

    let _a = Some(5);
    println!("{:?}", _a);
    let _b: Option<i32> = None;
    println!("{:?}", _b); 
}

