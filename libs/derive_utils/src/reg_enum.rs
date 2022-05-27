use proc_macro2::{Span, TokenStream, TokenTree};
use quote::quote;
use syn::{Data, DeriveInput, Fields, Type};

pub fn derive(input: DeriveInput) -> Result<TokenStream, (String, Span)> {
    let fields = match input.data {
        Data::Enum(e) => e,
        _ => return Err((("Enums not supported").into(), input.ident.span())),
    };

    let mut quote_fields = Vec::new();
    let mut unknown_ty: Option<Type> = None;
    for field in fields.variants {
        if field.ident.to_string() == "Unknown" {
            let field = match field.fields {
                Fields::Unnamed(fields) => match fields.unnamed.into_iter().next() {
                    Some(field) => field,
                    None => return Err(("Missing field".into(), field.ident.span())),
                },
                _ => {
                    return Err((
                        "Expected an unnamed field (e.g. `Unknown(u8)`)".into(),
                        field.ident.span(),
                    ))
                }
            };
            unknown_ty = Some(field.ty);
            continue;
        }
        let mut idx = None;
        for attr in field.attrs {
            let mut segments = attr.path.segments.iter();
            let segment = match segments.next() {
                Some(s) => s,
                None => continue,
            };
            if segment.ident.to_string() != String::from("doc") {
                continue;
            }

            let mut tokens = attr.tokens.into_iter();
            match tokens.next() {
                Some(TokenTree::Punct(p)) if p.as_char() == '=' => {}
                _ => continue,
            }
            let text = match tokens.next() {
                Some(TokenTree::Literal(lit)) => lit.to_string(),
                _ => continue,
            };
            let mut text = text.trim();
            if text.starts_with('"') && text.ends_with('"') && text.len() > 2 {
                text = text[1..text.len() - 2].trim();
            }
            if let Some(first_non_hex_digit) = text.bytes().position(|b| b == b' ' || b == b':') {
                let num_str = &text[..first_non_hex_digit];
                if num_str.starts_with("0x") {
                    if let Ok(num) = u64::from_str_radix(&num_str[2..], 16) {
                        idx = Some(num);
                    }
                } else if num_str.starts_with("0b") {
                    if let Ok(num) = u64::from_str_radix(&num_str[2..], 2) {
                        idx = Some(num);
                    }
                } else if let Ok(num) = num_str.parse() {
                    idx = Some(num);
                }
            }
        }
        let idx = match idx {
            Some(idx) => idx,
            None => return Err(("Could not find value, please start the documentation with 0x... to indicate what the value of this variant is".into(), field.ident.span())),
        };
        let ident = field.ident;
        quote_fields.push(quote! {
            #idx => Self::#ident,
        });
    }
    let unknown_ty = match unknown_ty {
        Some(ty) => ty,
        None => {
            return Err((
                "Missing variant `Unknown(u8)` (or a similar value)".into(),
                input.ident.span(),
            ))
        }
    };
    quote_fields.push(quote! {
        x => Self::Unknown(x as #unknown_ty)
    });
    let name = input.ident;
    Ok(quote! {
        impl #name {
            pub fn new(val: u64) -> Self {
                match val {
                    #( #quote_fields )*
                }
            }
        }
    })
}
