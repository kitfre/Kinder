extern crate proc_macro;
extern crate syn;
#[macro_use] extern crate quote;

use proc_macro::TokenStream;
use syn::VariantData;
use quote::Tokens;

#[proc_macro_derive(HigherOrder)]
pub fn derive_higher_order(input: TokenStream) -> TokenStream {
    // Construct a string representation
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).expect("Parsing ast with syn");


    // Build the impl
    let gen = impl_derive_higher_order(&ast);

    // Return the generated impl
    gen.parse().expect("Issue with parsing return from impl_derive_higher_order")
}

#[proc_macro_derive(Functor)]
pub fn derive_functor(input: TokenStream) -> TokenStream {
    // Construct a string representation
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).expect("Parsing ast with syn");


    // Build the impl
    let gen = impl_derive_functor(&ast);

    // Return the generated impl
    gen.parse().expect("Issue with parsing return from impl_derive_functor")
}

fn impl_derive_higher_order(ast: &syn::DeriveInput) -> Tokens {
    //first make sure we have an actual higher order type
    let generic_types = &ast.generics.ty_params;
    if generic_types.is_empty()
    {
        panic!("Cannot derive `HigherOrder` on type with no generic type parameters");
    }
    else if generic_types.len() > 1 {
        panic!("Cannot derive `HigherOrder` on type with more than one generic type parameter")
    }

    let name = &ast.ident;
    quote! {
        impl<A, C> HigherOrder<A> for #name<C> {
            type B = C;
            type C = #name<A>;
        }
    }
}

fn impl_derive_functor(ast: &syn::DeriveInput) -> Tokens {
    let name = &ast.ident;

    //first let's derive Functor for enums
    impl_derive_functor_enum(name, &ast.body)
}

fn impl_derive_functor_enum(name: &syn::Ident, enm: &syn::Body) -> Tokens {
    //assert that this is an enum and grab the variants
    let variants = match *enm {
        syn::Body::Struct(..) => panic!("Struct passed to derive functor for Enum"),
        syn::Body::Enum(ref v) => v
    };

    let mut tokens_vec: Vec<Tokens>  = Vec::new();

    for var in variants.into_iter() {
        let data = derive_functor_enum_variant(name, var);
        tokens_vec.push(data);
    }

    let tokens = quote! {
        impl<A, B> Functor<A> for #name<B> {
            fn fmap<F>(&self, f: F) -> #name<A> where F: Fn(&B) -> A {
                match *self {
                    #(#tokens_vec),*
                }
            }
        }
    };
    
    //println!("{:?}", tokens);
    tokens
}

fn derive_functor_enum_variant(name: &syn::Ident, var: &syn::Variant) -> Tokens {
    let id = &var.ident;

    //match on the data
    
    let data = match var.data {
        VariantData::Tuple(ref fields) => functor_tuple(name, id, fields), 
        VariantData::Unit => quote! { #name::#id => #name::#id },
        _ => quote! {}

    };

    data
}

fn functor_tuple(name: &syn::Ident, id: &syn::Ident, fields: &Vec<syn::Field>) -> Tokens {
    //apply f to any tuple fields of the right type
    let mut mapped: Vec<Tokens> = Vec::new();
    let mut unmapped: Vec<Tokens> = Vec::new();
    for (idx, field) in fields.into_iter().enumerate() {
        let mut tok = Tokens::new();
        let curr = format!("x{}", idx);
        tok.append(curr);
        let tok = tok;
        match field.ty {
            syn::Ty::Path(_, ref p) => {
                //need to check if it's a Vec
                let ident = &p.segments[0].ident;
                if ident == &syn::Ident::from("Vec") {
                    mapped.push(quote! { #tok.iter().map(f).collect() })
                } else {
                    mapped.push(quote!{ f(#tok) });
                }
                unmapped.push(quote!{ ref #tok });
            },
            syn::Ty::Slice(_) => {
                mapped.push(quote!{ })
            }
            _ => (),
        }
    }
    quote!{ #name::#id(#(#unmapped),*) => #name::#id(#(#mapped),*) }
}