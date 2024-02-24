extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Type};

#[proc_macro_derive(SerBytes)]
pub fn serializable_struct(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident; // The struct's name
    // todo: 
    let buffer_size: usize = 256; // Example buffer size, adjust based on your struct

    let serialize_code = match &input.data {
        Data::Struct(data) => {
            let field_serializations = data.fields.iter().map(|f| {
                let name = &f.ident;
                match &f.ty {
                    // Example serialization for different types
                    Type::Path(type_path) if type_path.path.is_ident("u32") => quote! {
                        {
                            let bytes = self.#name.to_le_bytes();
                            buffer[offset..offset + 4].copy_from_slice(&bytes);
                            offset += 4;
                        }
                    },
                    Type::Path(type_path) if type_path.path.is_ident("f32") => quote! {
                        {
                            let bytes = self.#name.to_le_bytes();
                            buffer[offset..offset + 4].copy_from_slice(&bytes);
                            offset += 4;
                        }
                    },
                    Type::Path(type_path) if type_path.path.is_ident("i16") => quote! {
                        {
                            let bytes = self.#name.to_le_bytes();
                            buffer[offset..offset + 2].copy_from_slice(&bytes);
                            offset += 2;
                        }
                    },
                    // Assuming all enums can be safely cast to u8
                    _ => quote! {
                        buffer[offset] = self.#name as u8;
                        offset += 1;
                    },
                }
            });
            quote! {
                let mut buffer = [0u8; #buffer_size];
                let mut offset = 0usize;
                #(#field_serializations)*
                core::result::Result::Ok((buffer, offset)) // Return the buffer and the used size
            }
        },
        _ => quote! { core::result::Result::Err("Only structs can be serialized") },
    };

    let expanded = quote! {
        impl #name {
            pub fn serialize(&self) -> core::result::Result<([u8; #buffer_size], usize), &'static str> {
                #serialize_code
            }

            // Placeholder for deserialization function
            // Actual implementation would need to parse the byte slice
            pub fn deserialize(_bytes: &[u8]) -> core::result::Result<Self, &'static str> {
                unimplemented!("Deserialization logic goes here")
            }
        }
    };

    TokenStream::from(expanded)
}
