// Not in the standard library, note that I'm pulling this package from the 
// web via crates.io, it needed to be listed in Cargo.toml dependecies
use rand::Rng;

// Can be used like classic enum:
enum Color {
	Red,
	Green,
	Blue,
}

// Can be used to solve the lack of null
fn maybe_get_color() -> Option<Color> {
	let chance = rand::thread_rng().gen_range(0..100);

	println!("Generated: {}", chance);

	// match is an expression
	match chance {
		0..=24 => Some(Color::Red), // Option::Some, but included by default
		25..=49 => Some(Color::Green),
		50..=74 => Some(Color::Blue),
		_ => None,    // Option::None, but included by default
	}
}

fn main() {
	let maybe_col = maybe_get_color();
	match maybe_col {
		// Can provide a pattern to match and destructure the Option returned
		Some(c) => match c {
			Color::Red => println!("Got red!"),
			Color::Green => println!("Got green!"),
			Color::Blue => println!("Got blue!"),
		},
		None => println!("Got nothing :("),
	}
	// my code
	let maybe_fruit = maybe_get_fruit();
	match maybe_fruit {
		Some(f) => match f {
			Fruit::Apple => println!("Picked an apple!"),
			Fruit::Banana => println!("Picked a banana!"),
			Fruit::Cherry => println!("Picked a cherry!"),
		},
		None => println!("Picked nothing :("),
	}
}

//TODO: Write your own enum, and use it with another match statement

enum Fruit {
	Apple,
	Banana,
	Cherry,
}

fn maybe_get_fruit() -> Option<Fruit> {
	let chance = rand::thread_rng().gen_range(0..100);

	println!("Generated: {}", chance);

	match chance {
		0..=24 => Some(Fruit::Apple),
		25..=49 => Some(Fruit::Banana),
		50..=74 => Some(Fruit::Cherry),
		_ => None,
	}
}
