use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use serde::Deserialize;
use syn::LitInt;

#[derive(Deserialize)]
pub struct EntityType {
    pub id: u16,
    pub max_health: Option<f32>,
    pub attackable: Option<bool>,
    pub summonable: bool,
    pub fire_immune: bool,
    pub dimension: [f32; 2],
    pub eye_height: f32,
}

impl ToTokens for EntityType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let id = LitInt::new(&self.id.to_string(), proc_macro2::Span::call_site());

        let max_health = match self.max_health {
            Some(mh) => quote! { Some(#mh) },
            None => quote! { None },
        };

        let attackable = match self.attackable {
            Some(a) => quote! { Some(#a) },
            None => quote! { None },
        };

        let summonable = self.summonable;
        let fire_immune = self.fire_immune;
        let eye_height = self.eye_height;

        let dimension0 = self.dimension[0];
        let dimension1 = self.dimension[1];

        tokens.extend(quote! {
            EntityType {
                id: #id,
                max_health: #max_health,
                attackable: #attackable,
                summonable: #summonable,
                fire_immune: #fire_immune,
                dimension: [#dimension0, #dimension1], // Correctly construct the array
                eye_height: #eye_height,
            }
        });
    }
}

pub(crate) fn build() -> TokenStream {
    println!("cargo:rerun-if-changed=../assets/entities.json");

    let json: HashMap<String, EntityType> =
        serde_json::from_str(include_str!("../../assets/entities.json"))
            .expect("Failed to parse entities.json");

    let mut consts = TokenStream::new();
    let mut type_from_raw_id_arms = TokenStream::new();
    let mut type_from_name = TokenStream::new();

    for (name, entity) in json.iter() {
        let id = entity.id as u8;
        let id_lit = LitInt::new(&id.to_string(), proc_macro2::Span::call_site());
        let upper_name = format_ident!("{}", name.to_uppercase());

        let entity_tokens = entity.to_token_stream();

        consts.extend(quote! {
            pub const #upper_name: EntityType = #entity_tokens;
        });

        type_from_raw_id_arms.extend(quote! {
            #id_lit => Some(Self::#upper_name),
        });

        type_from_name.extend(quote! {
            #name => Some(Self::#upper_name),
        });
    }
    quote! {
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub struct EntityType {
            pub id: u16,
            pub max_health: Option<f32>,
            pub attackable: Option<bool>,
            pub summonable: bool,
            pub fire_immune: bool,
            pub dimension: [f32; 2],
            pub eye_height: f32,
        }

        impl EntityType {
            #consts

            pub const fn from_raw(id: u16) -> Option<Self> {
                match id {
                    #type_from_raw_id_arms
                    _ => None
                }
            }

            pub fn from_name(name: &str) -> Option<Self> {
                match name {
                    #type_from_name
                    _ => None
                }
            }
        }
    }
}
