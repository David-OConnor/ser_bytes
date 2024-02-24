extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Type};

#[proc_macro_derive(SerBytes)]
pub fn serializable_struct(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident; // The struct's name

    // Here we'll calculate sizes based on field types statically
    let mut total_size = 0usize;

    if let Data::Struct(data) = &input.data {
        for f in data.fields.iter() {
            total_size += match &f.ty {
                syn::Type::Path(type_path) if type_path.path.is_ident("u8") => 1,
                syn::Type::Path(type_path) if type_path.path.is_ident("i8") => 1,
                syn::Type::Path(type_path) if type_path.path.is_ident("u16") => 2,
                syn::Type::Path(type_path) if type_path.path.is_ident("i16") => 2,
                syn::Type::Path(type_path) if type_path.path.is_ident("u32") => 4,
                syn::Type::Path(type_path) if type_path.path.is_ident("i32") => 4,
                syn::Type::Path(type_path) if type_path.path.is_ident("u64") => 8,
                syn::Type::Path(type_path) if type_path.path.is_ident("i64") => 8,
                syn::Type::Path(type_path) if type_path.path.is_ident("u128") => 16,
                syn::Type::Path(type_path) if type_path.path.is_ident("i128") => 16,
                syn::Type::Path(type_path) if type_path.path.is_ident("f32") => 4,
                syn::Type::Path(type_path) if type_path.path.is_ident("f64") => 8,
                // Assuming enums can be represented as u8
                _ => 1,
            };
        }
    }

    let serialize_code = match &input.data {
        Data::Struct(data) => {
            let field_serializations = data.fields.iter().map(|f| {
                let name = &f.ident;
                match &f.ty {
                    // todo: DRY
                    Type::Path(type_path) if type_path.path.is_ident("u8") => quote! {
                        {
                            buffer[offset] = self.#name;
                            offset += 1;
                        }
                    },
                    Type::Path(type_path) if type_path.path.is_ident("i8") => quote! {
                        {
                            buffer[offset] = self.#name as u8;
                            offset += 1;
                        }
                    },
                    Type::Path(type_path) if type_path.path.is_ident("u16") => quote! {
                        {
                            let bytes = self.#name.to_le_bytes();
                            buffer[offset..offset + 2].copy_from_slice(&bytes);
                            offset += 2;
                        }
                    },
                    Type::Path(type_path) if type_path.path.is_ident("i16") => quote! {
                        {
                            let bytes = self.#name.to_le_bytes();
                            buffer[offset..offset + 2].copy_from_slice(&bytes);
                            offset += 2;
                        }
                    },
                    Type::Path(type_path) if type_path.path.is_ident("u32") => quote! {
                        {
                            let bytes = self.#name.to_le_bytes();
                            buffer[offset..offset + 4].copy_from_slice(&bytes);
                            offset += 4;
                        }
                    },
                    Type::Path(type_path) if type_path.path.is_ident("i32") => quote! {
                        {
                            let bytes = self.#name.to_le_bytes();
                            buffer[offset..offset + 4].copy_from_slice(&bytes);
                            offset += 4;
                        }
                    },
                    Type::Path(type_path) if type_path.path.is_ident("u64") => quote! {
                        {
                            let bytes = self.#name.to_le_bytes();
                            buffer[offset..offset + 8].copy_from_slice(&bytes);
                            offset += 8;
                        }
                    },
                    Type::Path(type_path) if type_path.path.is_ident("i64") => quote! {
                        {
                            let bytes = self.#name.to_le_bytes();
                            buffer[offset..offset + 8].copy_from_slice(&bytes);
                            offset += 8;
                        }
                    },
                    Type::Path(type_path) if type_path.path.is_ident("u128") => quote! {
                        {
                            let bytes = self.#name.to_le_bytes();
                            buffer[offset..offset + 16].copy_from_slice(&bytes);
                            offset += 16;
                        }
                    },
                    Type::Path(type_path) if type_path.path.is_ident("i128") => quote! {
                        {
                            let bytes = self.#name.to_le_bytes();
                            buffer[offset..offset + 16].copy_from_slice(&bytes);
                            offset += 16;
                        }
                    },
                    Type::Path(type_path) if type_path.path.is_ident("f32") => quote! {
                        {
                            let bytes = self.#name.to_le_bytes();
                            buffer[offset..offset + 4].copy_from_slice(&bytes);
                            offset += 4;
                        }
                    },
                    Type::Path(type_path) if type_path.path.is_ident("f64") => quote! {
                        {
                            let bytes = self.#name.to_le_bytes();
                            buffer[offset..offset + 8].copy_from_slice(&bytes);
                            offset += 8;
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
                let mut buffer = [0u8; #total_size];
                let mut offset = 0usize;
                #(#field_serializations)*
                // core::result::Result::Ok((buffer, offset)) // Return the buffer and the used size
                buffer
            }
        },
        // _ => quote! { core::result::Result::Err("Only structs can be serialized") },
        _ => quote! { panic!("Only structs can be serialized") },
    };

    let expanded = quote! {
        impl #name {
            // pub fn serialize(&self) -> core::result::Result<([u8; #total_size], usize), &'static str> {
            pub fn serialize(&self) -> [u8; #total_size] {
                #serialize_code

                // let mut buffer = [0u8; #total_size];
                // let mut offset = 0usize;
                // // Serialization logic...
                // core::result::Result::Ok((buffer, offset))
            }

            // Placeholder for deserialization function
            // Actual implementation would need to parse the byte slice
            // pub fn deserialize(_bytes: &[u8]) -> core::result::Result<Self, &'static str> {
            pub fn deserialize(_bytes: &[u8]) -> core::result::Result<Self, &'static str> {
                unimplemented!("Deserialization logic goes here")
            }
        }
    };

    TokenStream::from(expanded)
}
