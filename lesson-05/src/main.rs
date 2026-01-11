
/// Prints the length of a string.
/// 
/// # Arguments
/// 
/// - `s` (`&String`) - Read only reference of s.
/// 
/// # Examples
/// 
/// ```
/// let _ = print_length();
/// ```
fn print_length(s: &String) {
    println!("{}", s.len());
}

/// Adds '!' to the end of string.
/// 
/// # Arguments
/// 
/// - `s` (`String`) - a String that the user wishes to manipulate.
/// 
/// # Returns
/// 
/// - `String` - Returns s + !.
/// 
/// # Examples
/// 
/// ```
/// let _ = update(s);
/// 
/// ```
/// OUTPUT: "s!"
fn update(s: String) -> String {
    s + "!"
}

fn main() {
    let s = String::from("Hello");
    let s = update(s);
    println!("{}", s);
}