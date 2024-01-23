use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::models::MyFieldReceiver;

pub struct NumericTraitToken(TokenStream);

impl ToTokens for NumericTraitToken {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens);
    }
}

impl MyFieldReceiver {
    pub fn numeric_trait_token(&self) -> NumericTraitToken {
        let numeric_trait = if self.is_numeric() {
            quote!(
                impl #field_impl_generics #crate_name::SetterNumeric<#field_type> for self::#field_name_as_camel
                #field_where_clause {}

                impl ::std::convert::From<self::#field_name_as_camel> for #crate_name::NumberLike {
                    fn from(val: self::#field_name_as_camel) -> Self {
                        val.0.into()
                    }
                }

                impl ::std::convert::From<&self::#field_name_as_camel> for #crate_name::NumberLike {
                    fn from(val: &self::#field_name_as_camel) -> Self {
                        val.clone().0.into()
                    }
                }

                impl<T: ::std::convert::Into<#crate_name::NumberLike>> ::std::ops::Add<T> for #field_name_as_camel {
                    type Output = #crate_name::Operation;

                    fn add(self, rhs: T) -> Self::Output {
                        let rhs: #crate_name::NumberLike = rhs.into();

                        #crate_name::Operation {
                                query_string: format!("{} + {}", self.build(), rhs.build()),
                                bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
                                errors: vec![],
                            }
                        }
                }

                impl<T: ::std::convert::Into<#crate_name::NumberLike>> ::std::ops::Sub<T> for #field_name_as_camel {
                    type Output = #crate_name::Operation;

                    fn sub(self, rhs: T) -> Self::Output {
                        let rhs: #crate_name::NumberLike = rhs.into();

                        #crate_name::Operation {
                            query_string: format!("{} - {}", self.build(), rhs.build()),
                            bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
                            errors: vec![],
                        }
                    }
                }

                impl<T: ::std::convert::Into<#crate_name::NumberLike>> ::std::ops::Mul<T> for #field_name_as_camel {
                    type Output = #crate_name::Operation;

                    fn mul(self, rhs: T) -> Self::Output {
                        let rhs: #crate_name::NumberLike = rhs.into();

                        #crate_name::Operation {
                            query_string: format!("{} * {}", self.build(), rhs.build()),
                            bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
                            errors: vec![],
                        }
                    }
                }

                impl<T: ::std::convert::Into<#crate_name::NumberLike>> ::std::ops::Div<T> for #field_name_as_camel {
                    type Output = #crate_name::Operation;

                    fn div(self, rhs: T) -> Self::Output {
                        let rhs: #crate_name::NumberLike = rhs.into();

                        #crate_name::Operation {
                            query_string: format!("{} / {}", self.build(), rhs.build()),
                            bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
                            errors: vec![],
                        }
                    }
                }

                impl<T: ::std::convert::Into<#crate_name::NumberLike>> ::std::ops::Add<T> for &#field_name_as_camel {
                    type Output = #crate_name::Operation;

                    fn add(self, rhs: T) -> Self::Output {
                        let rhs: #crate_name::NumberLike = rhs.into();

                        #crate_name::Operation {
                                query_string: format!("{} + {}", self.build(), rhs.build()),
                                bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
                                errors: vec![],
                            }
                        }
                }

                impl<T: ::std::convert::Into<#crate_name::NumberLike>> ::std::ops::Sub<T> for &#field_name_as_camel {
                    type Output = #crate_name::Operation;

                    fn sub(self, rhs: T) -> Self::Output {
                        let rhs: #crate_name::NumberLike = rhs.into();

                        #crate_name::Operation {
                            query_string: format!("{} - {}", self.build(), rhs.build()),
                            bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
                            errors: vec![],
                        }
                    }
                }

                impl<T: ::std::convert::Into<#crate_name::NumberLike>> ::std::ops::Mul<T> for &#field_name_as_camel {
                    type Output = #crate_name::Operation;

                    fn mul(self, rhs: T) -> Self::Output {
                        let rhs: #crate_name::NumberLike = rhs.into();

                        #crate_name::Operation {
                            query_string: format!("{} * {}", self.build(), rhs.build()),
                            bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
                            errors: vec![],
                        }
                    }
                }

                impl<T: ::std::convert::Into<#crate_name::NumberLike>> ::std::ops::Div<T> for &#field_name_as_camel {
                    type Output = #crate_name::Operation;

                    fn div(self, rhs: T) -> Self::Output {
                        let rhs: #crate_name::NumberLike = rhs.into();

                        #crate_name::Operation {
                            query_string: format!("{} / {}", self.build(), rhs.build()),
                            bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
                            errors: vec![],
                        }
                    }
                }
            )
        } else {
            quote!()
        };
    }
}
