
/// prints a string.
/// 
/// # Arguments
/// 
/// - `s` (`&String`) - Takes the reference to the value `s`.
/// 
/// # Examples
/// 
/// ```
/// let _ = take_string();
/// ```
fn take_string(s: &String) {
    println!("{}", s);
}

/// Adds an '!' to the end of string.
/// 
/// # Arguments
/// 
/// - `s` (`&mut String`) - Takes the mutable reference to a variable.
/// 
/// # Examples
/// 
/// ```
/// let _ = add_exclamation(s);
/// ```
fn add_exclamation(s: &mut String) {
    s.push('!');
}


fn main() {
    let mut s = String::from("Hello");
    add_exclamation(&mut s);
    println!("{}", s)
}