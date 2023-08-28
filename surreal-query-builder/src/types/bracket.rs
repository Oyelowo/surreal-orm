use crate::{Buildable, Erroneous, Parametric, ValueLike};

/// A code block. Surrounds the code with brackets.
pub struct Bracket(ValueLike);

/// Wraps the code in brackets.
pub fn bracket(code: impl Into<ValueLike>) -> Bracket {
    Bracket(code.into())
}

impl Buildable for Bracket {
    fn build(&self) -> String {
        format!("({})", self.0.build())
    }
}

impl From<ValueLike> for Bracket {
    fn from(code: ValueLike) -> Self {
        Self(code)
    }
}

impl From<Bracket> for ValueLike {
    fn from(bracket: Bracket) -> Self {
        ValueLike {
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
