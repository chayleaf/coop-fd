extern crate proc_macro;
use std::collections::HashMap;

use proc_macro2::{TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn::{Data, DeriveInput, Fields, Ident, Meta, Type};

enum FieldKind {
    Default,
    SpecialTag,
    SpecialFps,
    TagPath(Vec<TokenTree>),
}

struct FieldInfo {
    kind: FieldKind,
    ty: Type,
}

fn derive_ffd2(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse2(input).unwrap();
    let mut fields = HashMap::<Ident, FieldInfo>::new();
    let name = input.ident;
    match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(x) => {
                for field in x.named {
                    let mut field_kind = FieldKind::Default;
                    for attr in field.attrs {
                        if attr.meta.path().to_token_stream().to_string() != "ffd" {
                            continue;
                        }
                        match attr.meta {
                            Meta::List(x) => {
                                let mut tokens = x.tokens.into_iter();
                                let name = tokens.next().expect("unwrap name");
                                let eq = tokens.next().expect("unwrap eq").to_string();
                                assert_eq!(eq, "=");
                                match name.to_string().as_str() {
                                    "special" => {
                                        match tokens
                                            .next()
                                            .expect("special name")
                                            .to_string()
                                            .as_str()
                                        {
                                            "\"tag\"" => {
                                                assert!(tokens.next().is_none());
                                                field_kind = FieldKind::SpecialTag;
                                            }
                                            "\"fps\"" => {
                                                assert!(tokens.next().is_none());
                                                field_kind = FieldKind::SpecialFps;
                                            }
                                            x => {
                                                panic!("#[derive(Ffd)]: invalid special value: {x}")
                                            }
                                        }
                                    }
                                    "tag" => {
                                        let mut id = Vec::new();
                                        for tok in tokens {
                                            id.push(tok);
                                        }
                                        field_kind = FieldKind::TagPath(id);
                                    }
                                    _ => panic!("#[derive(Ffd)]: non-special/tag attr key"),
                                }
                            }
                            _ => panic!("#[derive(Ffd)]: non-Meta::List"),
                        }
                    }
                    assert!(fields
                        .insert(
                            field.ident.expect("field ident"),
                            FieldInfo {
                                kind: field_kind,
                                ty: field.ty,
                            },
                        )
                        .is_none());
                }
            }
            _ => panic!("#[derive(Ffd)] is only allowed on named fields"),
        },
        _ => panic!("#[derive(Ffd)] is only allowed on structs"),
    };
    let mut init_fields = TokenStream::new();
    let mut set_fields = TokenStream::new();
    let mut init_tag_stream = TokenStream::new();
    let mut set_tag_stream = TokenStream::new();
    let mut set_fps_stream = TokenStream::new();
    for (name, info) in fields.into_iter() {
        let is_option = info.ty.to_token_stream().to_string().starts_with("Option ");
        let is_vec = info.ty.to_token_stream().to_string().starts_with("Vec ")
            && !info.ty.to_token_stream().to_string().contains(" u8 ");
        assert!(!is_option || !is_vec);
        match info.kind {
            FieldKind::Default => init_fields.extend(quote! {
                #name: Default::default(),
            }),
            FieldKind::SpecialTag => {
                init_fields.extend(quote! {
                    #name: tag.try_into()?,
                });
                init_tag_stream.extend(quote! {
                    let mut cursor = ::std::io::Cursor::new(bytes);
                    let mut tag = [0u8, 0u8];
                    std::io::Read::read_exact(&mut cursor, &mut tag).map_err(|_| fiscal_data::Error::Eof)?;
                    let tag = u16::from_le_bytes(tag);
                    let mut len = [0u8, 0u8];
                    std::io::Read::read_exact(&mut cursor, &mut len).map_err(|_| fiscal_data::Error::Eof)?;
                    let len = u16::from_le_bytes(len);
                    let mut bytes = vec![0u8; len.into()];
                    std::io::Read::read_exact(&mut cursor, &mut bytes).map_err(|_| fiscal_data::Error::Eof)?;
                    let mut rest = vec![];
                    std::io::Read::read_to_end(&mut cursor, &mut rest)?;
                });
                set_tag_stream.extend(quote! {
                    ret.extend_from_slice(&u16::from(self.#name).to_le_bytes());
                    ret.extend_from_slice(&u16::try_from(ret.len() - 2).map_err(|_| fiscal_data::Error::InvalidLength)?.to_le_bytes());
                    ret.rotate_right(4);
                });
            }
            FieldKind::SpecialFps => {
                if is_option {
                    init_fields.extend(quote! {
                        #name: if rest.is_empty() {
                            None
                        } else {
                            Some(rest.try_into().map_err(|_| fiscal_data::Error::InvalidLength)?)
                        },
                    });
                    set_fps_stream.extend(quote! {
                        if let Some(x) = self.#name {
                            ret.extend_from_slice(&x);
                        }
                    });
                } else {
                    init_fields.extend(quote! {
                        #name: rest.try_into().map_err(|_| fiscal_data::Error::InvalidLength)?,
                    });
                    set_fps_stream.extend(quote! {
                        ret.extend_from_slice(&x);
                    });
                }
            }
            FieldKind::TagPath(path) => {
                let path = path.into_iter().map(|x| x.to_token_stream()).fold(
                    TokenStream::new(),
                    |mut a, b| {
                        a.extend(b);
                        a
                    },
                );
                if is_option {
                    init_fields.extend(quote! {
                        #name: obj.get::<#path>()?.map(|x| x.try_into()).transpose()?,
                    });
                    set_fields.extend(quote! {
                        if let Some(x) = self.#name {
                            obj.set::<#path>(x.try_into()?)?;
                        }
                    })
                } else if is_vec {
                    init_fields.extend(quote! {
                        #name: obj.get_all::<#path>()?.into_iter().map(|x| x.try_into()).collect::<Result<_, _>>()?,
                    });
                    set_fields.extend(quote! {
                        for x in self.#name {
                            obj.push::<#path>(x.try_into()?)?;
                        }
                    })
                } else {
                    init_fields.extend(quote! {
                        #name: obj.get::<#path>()?.ok_or(fiscal_data::Error::InvalidFormat)?.try_into()?,
                    });
                    set_fields.extend(quote! {
                        obj.set::<#path>(self.#name.try_into()?)?;
                    })
                }
            }
        }
    }
    let w = Ident::new(&format!("_ffd_impl_{}", name), name.span());
    let (impl_gen, ty_gen, wher) = input.generics.split_for_impl();
    quote! {
        #[allow(non_snake_case)]
        mod #w {
            use super::*;
            use std::io::Read;
            impl #impl_gen TryFrom<fiscal_data::Object> for super::#name #ty_gen #wher {
                type Error = fiscal_data::Error;
                fn try_from(obj: fiscal_data::Object) -> Result<Self, Self::Error> {
                    fiscal_data::TlvType::from_bytes(
                        fiscal_data::TlvType::into_bytes(obj)?,
                    )
                }
            }
            impl #impl_gen TryFrom<super::#name #ty_gen> for fiscal_data::Object #wher {
                type Error = fiscal_data::Error;
                fn try_from(obj: super::#name #ty_gen) -> Result<Self, Self::Error> {
                    fiscal_data::TlvType::from_bytes(
                        fiscal_data::TlvType::into_bytes(obj)?,
                    )
                }
            }
            impl #impl_gen fiscal_data::TlvType for super::#name #ty_gen #wher {
                fn from_bytes(bytes: Vec<u8>) -> fiscal_data::Result<Self> {
                    #init_tag_stream
                    let obj = fiscal_data::Object::from_bytes(bytes)?;
                    Ok(Self {
                        #init_fields
                    })
                }
                fn into_bytes(self) -> fiscal_data::Result<Vec<u8>> {
                    let mut obj = fiscal_data::Object::new();
                    #set_fields
                    let mut ret = obj.into_bytes()?;
                    #set_tag_stream
                    #set_fps_stream
                    Ok(ret)
                }
                const REPR: fiscal_data::internal::Repr = fiscal_data::internal::Repr::Object;
            }
        }
    }
}

#[proc_macro_derive(Ffd, attributes(ffd))]
pub fn derive_ffd(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_ffd2(input.into()).into()
}
