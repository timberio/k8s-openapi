#![recursion_limit = "1024"]
#![deny(rust_2018_idioms, warnings)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(
    clippy::too_many_lines,
)]

//! This crate contains the implementation of the proc macros in the [`k8s-openapi-derive`](https://crates.io/crates/k8s-openapi-derive) crate.

mod custom_resource_definition;

trait CustomDerive: Sized {
	fn parse(input: syn::DeriveInput, tokens: proc_macro2::TokenStream) -> Result<Self, syn::Error>;
	fn emit(self) -> Result<proc_macro2::TokenStream, syn::Error>;
}

fn run_custom_derive<T>(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream where T: CustomDerive {
	let tokens = input.clone();
	let token_stream = syn::parse2(input).and_then(|input| <T as CustomDerive>::parse(input, tokens)).and_then(<T as CustomDerive>::emit);
	match token_stream {
		Ok(token_stream) => token_stream,
		Err(err) => err.to_compile_error(),
	}
}

trait ResultExt<T> {
	fn spanning(self, spanned: impl quote::ToTokens) -> Result<T, syn::Error>;
}

impl<T, E> ResultExt<T> for Result<T, E> where E: std::fmt::Display {
	fn spanning(self, spanned: impl quote::ToTokens) -> Result<T, syn::Error> {
		self.map_err(|err| syn::Error::new_spanned(spanned, err))
	}
}

#[no_mangle]
pub fn derive_custom_resource_definition(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
	run_custom_derive::<custom_resource_definition::CustomResourceDefinition>(input)
}
