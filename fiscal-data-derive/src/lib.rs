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

fn derive_ffd2(doc: bool, input: TokenStream) -> TokenStream {
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
    let mut set_doc_fields = TokenStream::new();
    for (name, info) in fields {
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
                    #name: doc.tag().try_into()?,
                });
                set_doc_fields.extend(quote! {
                    doc.set_tag(self.#name.try_into()?);
                });
            }
            FieldKind::SpecialFps => {
                if is_option {
                    init_fields.extend(quote! {
                        #name: doc.message_fiscal_sign(),
                    });
                    set_doc_fields.extend(quote! {
                        if let Some(x) = self.#name {
                            doc.set_message_fiscal_sign(x);
                        }
                    });
                } else {
                    init_fields.extend(quote! {
                        #name: doc.message_fiscal_sign().ok_or(fiscal_data::Error::InvalidLength)?,
                    });
                    set_doc_fields.extend(quote! {
                        doc.set_message_fiscal_sign(x);
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
                    });
                } else if is_vec {
                    init_fields.extend(quote! {
                        #name: obj.get_all::<#path>()?.into_iter().map(|x| x.try_into()).collect::<Result<_, _>>()?,
                    });
                    set_fields.extend(quote! {
                        for x in self.#name {
                            obj.push::<#path>(x.try_into()?)?;
                        }
                    });
                } else {
                    init_fields.extend(quote! {
                        #name: obj.get::<#path>()?.ok_or(fiscal_data::Error::InvalidFormat)?.try_into()?,
                    });
                    set_fields.extend(quote! {
                        obj.set::<#path>(self.#name.try_into()?)?;
                    });
                }
            }
        }
    }
    let w = Ident::new(&format!("_ffd_impl_{name}"), name.span());
    let (impl_gen, ty_gen, wher) = input.generics.split_for_impl();
    let ty = if doc {
        syn::parse_str::<Ident>("Document")
    } else {
        syn::parse_str::<Ident>("Object")
    }
    .unwrap();
    let ret_name = if doc {
        syn::parse_str::<Ident>("doc")
    } else {
        syn::parse_str::<Ident>("obj")
    }
    .unwrap();
    let init1 = if doc {
        quote! {
            let doc = fiscal_data::Document::from_bytes(bytes)?;
            let obj = doc.data();
        }
    } else {
        quote! {
            let obj = fiscal_data::Object::from_bytes(bytes)?;
        }
    };
    let init2 = if doc {
        quote! {
            let mut doc = fiscal_data::Document::default();
            let obj = doc.data_mut();
        }
    } else {
        quote! {
            let mut obj = fiscal_data::Object::new();
        }
    };
    let extra_tryfrom = if doc {
        quote! {
            impl #impl_gen TryFrom<super::#name #ty_gen> for fiscal_data::Object #wher {
                type Error = fiscal_data::Error;
                fn try_from(doc: super::#name #ty_gen) -> Result<Self, Self::Error> {
                    Ok(<fiscal_data::Document as fiscal_data::TlvType>::from_bytes(
                        fiscal_data::TlvType::into_bytes(doc)?,
                    )?.into_data())
                }
            }
        }
    } else {
        quote! {}
    };
    quote! {
        #[allow(non_snake_case)]
        mod #w {
            use super::*;
            use std::io::Read;
            impl #impl_gen TryFrom<fiscal_data::#ty> for super::#name #ty_gen #wher {
                type Error = fiscal_data::Error;
                fn try_from(obj: fiscal_data::#ty) -> Result<Self, Self::Error> {
                    fiscal_data::TlvType::from_bytes(
                        fiscal_data::TlvType::into_bytes(obj)?,
                    )
                }
            }
            impl #impl_gen TryFrom<super::#name #ty_gen> for fiscal_data::#ty #wher {
                type Error = fiscal_data::Error;
                fn try_from(obj: super::#name #ty_gen) -> Result<Self, Self::Error> {
                    fiscal_data::TlvType::from_bytes(
                        fiscal_data::TlvType::into_bytes(obj)?,
                    )
                }
            }
            #extra_tryfrom
            impl #impl_gen fiscal_data::TlvType for super::#name #ty_gen #wher {
                fn from_bytes(bytes: Vec<u8>) -> fiscal_data::Result<Self> {
                    #init1
                    Ok(Self {
                        #init_fields
                    })
                }
                fn into_bytes(self) -> fiscal_data::Result<Vec<u8>> {
                    #init2
                    #set_fields
                    #set_doc_fields
                    #ret_name.into_bytes()
                }
                const REPR: fiscal_data::internal::Repr = fiscal_data::internal::Repr::#ty;
            }
        }
    }
}

#[proc_macro_derive(Ffd, attributes(ffd))]
pub fn derive_ffd(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_ffd2(false, input.into()).into()
}

#[proc_macro_derive(FfdDoc, attributes(ffd))]
pub fn derive_ffd_doc(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_ffd2(true, input.into()).into()
}
