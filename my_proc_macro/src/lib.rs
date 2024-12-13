extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

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


#[proc_macro_derive(MyInteger)]
pub fn my_integer(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl MyInteger for #name {
            fn succ(self) -> impl MyInteger {
                Succ(#name)
            }

            fn add<N: MyInteger>(self, other: N) -> impl MyInteger {
                other
            }

            fn mull<N: MyInteger>(self, other: N) -> impl MyInteger {
                #name
            }

            fn prev(self) -> impl MyInteger {
                Prev(#name)
            }

            fn neg(self) -> impl MyInteger {
                #name
            }

            fn sub<N: MyInteger>(self, other: N) -> impl MyInteger {
                other.neg()
            }

            fn to_int(self) -> i32 {
                0
            }
        }
    };
    gen.into()
}