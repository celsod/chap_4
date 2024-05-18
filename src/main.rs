fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!"); //push_str() appends a literal to a String

    println!("{}", s);

    let s1 = String::from("Hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
    
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    let s3 = String::from("hello");  // s comes into scope

    println!("s3 = {}!!!", s3);
    takes_ownership(s3);             // s's value moves into the function...
                                    // ... and so is no longer valid here
    //println!("s3 is now = {}", s3);

    let x = 5;                      // x comes into scope

    makes_copy(x);  // x would move into the function, but i32 is Copy, so it's okay to still use x afterward

    let s4 = String::from("hello");

    let len = calculate_length(&s4);

    println!("The length of '{}' is {}.", s4, len);

    let mut s5 = String::from("5th Hello.");

    change(&mut s5);
}


fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn calculate_length(s: &String) -> usize { //s is a reference to a string
    s.len()
} // Here, s goes out of scope.  But because it does not have ownership of what it refers to,
// it is not dropped

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}