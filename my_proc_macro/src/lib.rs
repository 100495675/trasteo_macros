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

        impl<N: MyInteger> PartialEq<Succ<N>> for #name {
            fn eq(&self, _other: &Succ<N>) -> bool {
                false
            }
        }

        impl<N: MyInteger> PartialEq<Prev<N>> for #name {
            fn eq(&self, _other: &Prev<N>) -> bool {
                false
            }
        }

        impl PartialEq for #name {
            fn eq(&self, _other: &Self) -> bool {
                true
            }
        }

        impl Eq for #name {}

        impl<N: MyInteger> PartialOrd<Prev<N>> for #name {
            fn partial_cmp(&self, _other: &Prev<N>) -> Option<Ordering> {
                Some(Ordering::Greater)
            }
        }

        impl PartialOrd for #name {
            fn partial_cmp(&self, _other: &#name) -> Option<Ordering> {
                Some(Ordering::Equal)
            }
        }

        impl<N: MyInteger> PartialOrd<Succ<N>> for #name {
            fn partial_cmp(&self, _other: &Succ<N>) -> Option<Ordering> {
                Some(Ordering::Less)
            }
        }

        impl Ord for #name {
            fn cmp(&self, other: &#name) -> Ordering {
                self.partial_cmp(other).unwrap()
            }
        }
    };
    gen.into()
}
