//! Refactor this code so that instead of passing `s` into the `create_string` function.
//! The String gets created in the function itself and passed back to the main
//! function.

fn main() {
    let mut s1 = create_string();

    println!("{} == `{}`", stringify!(s1), s1);

    s1.push_str(" World!");

    println!("{} == `{}`", stringify!(s1), s1);
}

/// `create_string()` nu mai primește un argument `s: String`
fn create_string() -> String {
    let mut s = String::from("Hello");  // Creăm stringul în funcție

    s
}

