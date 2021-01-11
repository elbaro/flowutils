extern crate proc_macro;
use proc_macro::TokenStream;

/// Golang's `defer` in Rust.
/// 
/// - This is scoped. A deferred action is performed at the end of the scope, not the end of the function.
/// - Multiple `defer!`s are executed in the reverse order.
///
/// Example:
/// ```rust
/// flowutils::defer!(println!("order 6"));
/// {
///     flowutils::defer!(println!("order 4"));
///     flowutils::defer!({
///         println!("order 2");
///         println!("order 3");
///     });
///     flowutils::defer!(println!("order 1"));
/// }
/// flowutils::defer!(println!("order 5"));
/// ```
#[proc_macro]
pub fn defer(token_stream: TokenStream) -> TokenStream {
    let expr: syn::Expr = syn::parse_macro_input!(token_stream as syn::Expr);
    let uid = uuid::Uuid::new_v4();
    let name = format!("__flowutils_{}", uid).replace('-', "_");
    let defer_guard_type = syn::Ident::new(&name, proc_macro2::Span::call_site());
    let defer_guard_var = syn::Ident::new(&format!("{}_", &name), proc_macro2::Span::call_site());
    (quote::quote!{
        struct #defer_guard_type;
        impl Drop for #defer_guard_type {
            fn drop(&mut self) {
                #expr
            }
        }
        let #defer_guard_var = #defer_guard_type;
    }).into()
    
}

fn parse_unwrap(input: syn::parse::ParseStream) -> syn::Result<(syn::Expr, syn::Arm)> {
    let expr:syn::Expr = input.parse()?;
    let _:  syn::Token![,] = input.parse()?;
    let arm: syn::Arm = input.parse()?;
    Ok((expr, arm))
}

/// A short version of `if let` when you already know the pattern.
///
/// Returns the inner value or panics.
/// For a complex enum variant, 
///
/// Example:
///
/// ```rust
/// enum T{
///    A(i32),
///    B(String, u64),
///    C{p: usize, q: f32, r: i8}
/// }
///
/// let some_enum = T::A(3);
/// let inner = flowutils::unwrap_pattern!(some_enum, T::A(x));
///
/// let some_enum = T::B(String::new("str"), 3);
/// let tuple: (u64, String) = flowutils::unwrap_pattern!(some_enum, T::B(var1, var2) => (var2, var1));
///
/// let some_enum = T::B(String::new("str"), 3);
/// let complex: usize = flowutils::unwrap_pattern!(some_enum, T::C{var3, _, var4}, var3);
/// ```
#[proc_macro]
pub fn unwrap_pattern(token_stream: TokenStream) -> TokenStream {
    let (expr, arm) = syn::parse_macro_input!(token_stream with parse_unwrap);

    (quote::quote!{
        match #expr {
            #arm,
            _ => panic!("unwrap(..) failed"),
        }
    }).into()
}

// #[derive(Debug, Clone)]
// struct TryPatternError;

// impl std::fmt::Display for TryPatternError {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "let() pattern-match failed")
//     }
// }


/// Similar to `unwrap_pattern` but returns Result<_, &str>.
///
/// Example:
///
/// ```rust
/// let a = try_pattern!(some_enum, A(x)=>x)?;
/// ```
///
/// Equivalent:
///
/// ```rust
/// let result = if let A(x) = some_enum {
///     Ok(x)
/// } else {
///     Err("failed")
/// };
/// let a = result?;
/// ```

#[proc_macro]
pub fn try_pattern(token_stream: TokenStream) -> TokenStream {
    let (expr, mut arm) = syn::parse_macro_input!(token_stream with parse_unwrap);
    let old_body = arm.body;

    // wrap arm.body in Ok()
    arm.body = syn::parse_quote! {
        Ok(#old_body)
    };
    
    (quote::quote!{
        match #expr {
            #arm,
            // TODO: _ => Err(flowutils::TryPatternError),
            _ => Err("try_pattern() failed"),
        }
    }).into()
}
