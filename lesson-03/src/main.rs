/// Takes in two 32 bit integers and returns the sum of them.
/// 
/// # Arguments
/// 
/// - `a` (`i32`) - Integer 1.
/// - `b` (`i32`) - Integer 2.
/// 
/// # Returns
/// 
/// - `i32` - returns a value in the type of integer 32.
/// 
/// # Examples
/// 
/// ```
/// let _ = add(2,3);
/// ```
fn add(a: i32, b: i32) -> i32 {
    a + b
}


/// Increments a value by 1.
/// 
/// # Arguments
/// 
/// - `x` (`i32`) - Takes in a integer of bit 32.
/// 
/// # Examples
/// 
/// ```
/// let _ = increment(x);
/// ```
fn increment(x: i32) -> i32 {
    x + 1
}

fn main() {
    let x = 5;
    let x = increment(x);
    println!("{}", x);
}
