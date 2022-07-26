// sol 1

// FILL in the blanks
fn main() {  
   let mut s = String::from("hello, world");

   let slice1: &str = &s; // in two ways
   assert_eq!(slice1, "hello, world");

   let slice2 = &s[0..5];
   assert_eq!(slice2, "hello");

   let slice3: &mut String = &mut s; 
   slice3.push('!');
   assert_eq!(slice3, "hello, world!");

   println!("Success!")
}


// sol 2

// FILL in the blanks
fn main() {  
	let mut s = String::from("hello, world");
 
	let slice1: &str = s.as_str(); // in two ways
	assert_eq!(slice1, "hello, world");
 
	let slice2 = &s[0..5];
	assert_eq!(slice2, "hello");
 
	let slice3: &mut String = &mut s; 
	slice3.push('!');
	assert_eq!(slice3, "hello, world!");
 
	println!("Success!")
 }
 