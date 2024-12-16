extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro]
pub fn from_int(input: TokenStream) -> TokenStream {
    let value: i32 = input.to_string().parse().expect("Failed to parse input");
    let cont = if value.is_positive() { "Succ" } else { "Prev" };
    let mut output = String::from("Zero");
    for _ in 0..value.abs() {
        output = format!("{}({})", cont, output);
    }
    output.parse().expect("Failed to parse output")
}
