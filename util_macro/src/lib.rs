extern crate proc_macro;

use proc_macro::{TokenStream};
use quote::{quote};
use syn::{parse_macro_input, DataStruct, Data, Fields, Field, DeriveInput};
use syn::token::Comma;

#[proc_macro_derive(Fields)]
pub fn fields_derive(input: TokenStream) -> TokenStream {
    let DeriveInput {ident, data, ..} = parse_macro_input!(input);
    let fields_punct = match data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => fields.named,
        _ => panic!("Only struct with name fields"),
    };
    let mut field_names: Vec<String> = Vec::new();

    for f in &fields_punct {
        field_names.push(f.ident.as_ref().unwrap().to_string());
    };

    let mut field_idents: Vec<&syn::Ident> = Vec::new();
    for f in &fields_punct {
        field_idents.push(f.ident.as_ref().unwrap());
    }

    let output = quote!{
        impl Printable for #ident {
            fn titles(&self) -> Vec<&str> {
                return vec![#(#field_names),*];
            }

            fn row(&self) -> Vec<String> {
                return vec![#(self.#field_idents.to_string()),*];
            }
        }
    };

    output.into()
}

#[proc_macro_derive(Param, attributes(id, parent_id, name, value))]
pub fn param_derive(input: TokenStream) -> TokenStream {
    let DeriveInput {ident, data, ..} = parse_macro_input!(input);
    let fields_punct = match data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => fields.named,
        _ => panic!("Only struct with name fields"),
    };

    let name = get_field(&fields_punct, "name".to_string());
    let value = get_field(&fields_punct, "value".to_string());
    let parent_id  = get_field(&fields_punct, "parent_id".to_string());

    let output = quote!{
        impl #ident {
            pub fn add(p_id: i32, n: &str, v: &str ) -> Result<()>{
                let mut conn = db_connect();
                diesel::insert_into(table)
                    .values((#name.eq(n), #value.eq(v), #parent_id.eq(p_id)))
                    .execute(&mut conn)?;

                Ok(())
            }
            pub fn del(the_id: i32) -> Result<()> {
                let mut conn = db_connect();
                diesel::delete(table)
                    .filter(id.eq(the_id))
                    .execute(&mut conn)?;
                Ok(())
            }

            pub fn update(the_id: i32, n: &str, v: &str) -> Result<()> {
                let mut conn = db_connect();
                diesel::update(table)
                    .filter(id.eq(the_id))
                    .set((#name.eq(n), #value.eq(v)))
                    .execute(&mut conn)?;

                Ok(())
            }

            pub fn belong_to(parent_id: i32) -> Result<Vec<#ident>> {
                let mut conn = db_connect();

                let results: Vec<#ident> = table
                    .filter(#parent_id.eq(parent_id))
                    .load(&mut conn)?;

                return Ok(results);
            }
        }
    };

    output.into()
}

#[proc_macro_derive(HashMapHelper)]
pub fn hashmap_helper_derive(input: TokenStream) -> TokenStream {
    let DeriveInput {ident, ..} = parse_macro_input!(input);

    let output = quote!{
        impl #ident {
            pub fn get(name: &str, map: &HashMap<String, #ident>) -> String {
                match map.get(name) {
                    Some(v) => v.value.to_string(),
                    None => "".to_string()
                }
            }

            pub fn update(name: &str, map: &mut HashMap<String, #ident>, form_data: &FormData) {
                let value = &form_data.get(name).as_string().unwrap();
                let old = map.get(name);
                
                let mut new = match old {
                    Some(h) => { #ident {
                        ..(*h).clone()
                    }}
                    _ => {#ident::new()}
                };
            
                new.name = name.to_string();
                new.value = value.to_string();
                map.insert(name.to_string(), new);                
            }
        }
    };

    output.into()
}

fn get_field(fields_punct: &syn::punctuated::Punctuated<Field, Comma>, a: String) -> Option<&syn::Ident>{
    for f in fields_punct {
        let attr = f
            .attrs.first();
        let attr = match attr {
            Some(attr) => attr,
            None => continue
        };

        let attr = attr.path
            .get_ident()
            .unwrap();
        if attr == &a {
            return Some(f.ident.as_ref().unwrap());
        }
    }

    None
}
