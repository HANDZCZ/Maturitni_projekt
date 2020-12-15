use mysql::prelude::*;
use mysql::Pool;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, parse_quote, Ident, ItemEnum};

#[proc_macro_attribute]
pub fn get_roles_from_db(attr: TokenStream, item: TokenStream) -> TokenStream {
    dotenv::dotenv().expect("Dotenv error");
    let mut parsed_enum = parse_macro_input!(item as ItemEnum);
    let int_type = parse_macro_input!(attr as Ident);

    let database_url = std::env::var("DATABASE_URL").expect("Missing env variable DATABASE_URL");

    let pool = Pool::new(database_url).expect("Couldn't create pool");
    let mut conn = pool.get_conn().expect("Couldn't get connection from pool");

    let res: Vec<(usize, String)> = conn
        .query("select id, name from roles")
        .expect("Couldn't get roles from db");

    let mut variants = Punctuated::new();
    for (id, name) in res {
        let name = Ident::new(name.as_str(), Span::call_site());
        let variant: syn::Variant = parse_quote! {
            #name = #id as #int_type
        };
        variants.push(variant);
    }

    parsed_enum.variants = variants;

    (quote! {
        #parsed_enum
    })
    .into()
}
