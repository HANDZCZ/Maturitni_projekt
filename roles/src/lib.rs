use postgres::{Client, NoTls};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, parse_quote, Ident, ItemEnum};

#[proc_macro_attribute]
pub fn get_roles_from_db(_attr: TokenStream, item: TokenStream) -> TokenStream {
    dotenv::dotenv().expect("Dotenv error");
    let mut parsed_enum = parse_macro_input!(item as ItemEnum);

    let database_url = std::env::var("DATABASE_URL").expect("Missing env variable DATABASE_URL");

    let mut client = Client::connect(&database_url, NoTls).expect("Couldn't create pool");

    let mut variants = Punctuated::new();
    for row in client
        .query("select name, id from roles", &[])
        .expect("Couldn't get roles from db")
    {
        let name = Ident::new(row.get(0), Span::call_site());
        let id: i16 = row.get(1);
        let variant: syn::Variant = parse_quote! {
            #name = #id as i32
        };
        variants.push(variant);
    }

    parsed_enum.variants = variants;

    (quote! {
        #parsed_enum
    })
    .into()
}
