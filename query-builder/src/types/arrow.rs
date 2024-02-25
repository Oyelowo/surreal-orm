use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum EdgeDirection {
    Out,
    In,
}

// #[derive(Debug, Clone, Copy)]
// struct Arrow(&'static str);
//
// impl Display for Arrow {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.0)
//     }
// }

impl EdgeDirection {
    /// Get the arrow symbol for the edge direction
    pub fn as_arrow_symbol(&self) -> Arrow {
        match self {
            EdgeDirection::Out => Arrow::Right,
            EdgeDirection::In => Arrow::Left,
        }
    }
}

// impl ToTokens for EdgeDirection {
//     fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
//         let arrow = self.as_arrow_symbol();
//         tokens.extend(arrow.parse::<proc_macro2::TokenStream>().unwrap());
//     }
// }

// impl From<EdgeDirection> for ::proc_macro2::TokenStream {
//     fn from(direction: EdgeDirection) -> Self {
//         match direction {
//             EdgeDirection::OutArrowRight => quote::quote!(->),
//             EdgeDirection::InArrowLeft => quote::quote!(<-),
//         }
//     }
// }

// impl From<EdgeDirection> for &str {
//     fn from(direction: EdgeDirection) -> Self {
//         direction.as_arrow_symbol()
//     }
// }
//
// impl From<&EdgeDirection> for String {
//     fn from(direction: &EdgeDirection) -> Self {
//         direction.as_arrow_symbol().into()
//     }
// }
//
// impl From<EdgeDirection> for String {
//     fn from(direction: EdgeDirection) -> Self {
//         direction.as_arrow_symbol().into()
//     }
// }

// impl ::std::fmt::Display for EdgeDirection {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let arrow = direction.as_arrow_symbol().into();
//         write!(f, "{arrow}")
//     }
// }

/// Arrow for relation edge direction
#[derive(Debug, Clone, Copy)]
pub enum Arrow {
    Right,
    Left,
}

impl Display for Arrow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let arrow_str = match self {
            Arrow::Right => "->",
            Arrow::Left => "<-",
        };
        write!(f, "{arrow_str}")
    }
}
