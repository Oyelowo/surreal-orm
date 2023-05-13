use crate::{statements::QueryChain, Buildable, Erroneous, Parametric, Valuex};

/// A code block. Surrounds the code with brackets.
pub struct Bracket(Valuex);

/// Wraps the code in brackets.
pub fn bracket(code: impl Into<Valuex>) -> Bracket {
    Bracket(code.into())
}

impl Buildable for Bracket {
    fn build(&self) -> String {
        format!("({})", self.0.build())
    }
}

impl From<Valuex> for Bracket {
    fn from(code: Valuex) -> Self {
        Self(code)
    }
}

impl From<Bracket> for Valuex {
    fn from(bracket: Bracket) -> Self {
        Valuex {
            string: bracket.build(),
            bindings: bracket.get_bindings(),
            errors: bracket.get_errors(),
        }
    }
}

impl Parametric for Bracket {
    fn get_bindings(&self) -> crate::BindingsList {
        self.0.get_bindings()
    }
}

impl Erroneous for Bracket {
    fn get_errors(&self) -> crate::ErrorList {
        self.0.get_errors()
    }
}
