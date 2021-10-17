//! `bae` is a crate for proc macro authors, which simplifies parsing of attributes. It is
//! heavily inspired by [`darling`](https://crates.io/crates/darling) but has a significantly
//! simpler API.
//!
//! ```rust
//! use bae::FromAttributes;
//!
//! #[derive(
//!     Debug,
//!     Eq,
//!     PartialEq,
//!
//!     // This will add two functions:
//!     // ```
//!     // fn from_attributes(attrs: &[syn::Attribute]) -> Result<MyAttr, syn::Error>
//!     // fn try_from_attributes(attrs: &[syn::Attribute]) -> Result<Option<MyAttr>, syn::Error>
//!     // ```
//!     //
//!     // `try_from_attributes` returns `Ok(None)` if the attribute is missing, `Ok(Some(_))` if
//!     // its there and is valid, `Err(_)` otherwise.
//!     FromAttributes,
//! )]
//! pub struct MyAttr {
//!     // Anything that implements `syn::parse::Parse` is supported.
//!     mandatory_type: syn::Type,
//!     mandatory_ident: syn::Ident,
//!
//!     // Fields wrapped in `Option` are optional and default to `None` if
//!     // not specified in the attribute.
//!     optional_missing: Option<syn::Type>,
//!     optional_given: Option<syn::Type>,
//!
//!     // A "switch" is something that doesn't take arguments.
//!     // All fields with type `Option<()>` are considered swiches.
//!     // They default to `None`.
//!     switch: Option<()>,
//! }
//!
//! // `MyAttr` is now equipped to parse attributes named `my_attr`. For example:
//! //
//! //     #[my_attr(
//! //         switch,
//! //         mandatory_ident = foo,
//! //         mandatory_type = SomeType,
//! //         optional_given = OtherType,
//! //     )]
//! //     struct Foo {
//! //         ...
//! //     }
//!
//! // the input and output type would normally be `proc_macro::TokenStream` but those
//! // types cannot be used outside the compiler itself.
//! fn my_proc_macro(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
//!     let item_struct = syn::parse2::<syn::ItemStruct>(input).unwrap();
//!
//!     let my_attr = MyAttr::from_attributes(&item_struct.attrs).unwrap();
//!
//!     assert_eq!(
//!         my_attr.mandatory_type,
//!         syn::parse_str::<syn::Type>("SomeType").unwrap()
//!     );
//!
//!     assert_eq!(my_attr.optional_missing, None);
//!
//!     assert_eq!(
//!         my_attr.optional_given,
//!         Some(syn::parse_str::<syn::Type>("OtherType").unwrap())
//!     );
//!
//!     assert_eq!(my_attr.mandatory_ident, syn::parse_str::<syn::Ident>("foo").unwrap());
//!
//!     assert_eq!(my_attr.switch.is_some(), true);
//!
//!     // ...
//!     #
//!     # quote::quote! {}
//! }
//! #
//! # fn main() {
//! #     let code = quote::quote! {
//! #         #[other_random_attr]
//! #         #[my_attr(
//! #             switch,
//! #             mandatory_ident = foo,
//! #             mandatory_type = SomeType,
//! #             optional_given = OtherType,
//! #         )]
//! #         struct Foo;
//! #     };
//! #     my_proc_macro(code);
//! # }
//! ```

pub use better_bae_macros::FromAttributes;

pub trait TryFromAttributes
where
    Self: Sized,
{
    fn attr_name() -> &'static str;

    fn try_from_attributes(attrs: &[syn::Attribute]) -> syn::Result<Option<Self>>;

    fn from_attributes(attrs: &[syn::Attribute]) -> syn::Result<Self> {
        if let Some(attr) = Self::try_from_attributes(attrs)? {
            Ok(attr)
        } else {
            Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                &format!("missing attribute `#[{}]`", Self::attr_name()),
            ))
        }
    }
}
