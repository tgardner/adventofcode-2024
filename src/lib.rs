pub mod template;

// Use this file to add helper functions and additional modules.
pub fn signum(n: i32) -> i32 {
    if n > 0 {
        1
    } else if n < 0 {
        -1
    } else {
        0
    }
}
