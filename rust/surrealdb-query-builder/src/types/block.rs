use surrealdb::sql;

use crate::{statements::QueryChain, Buildable, Erroneous, Parametric, Valuex};

/// A code block. Surrounds the code with curly braces.
pub fn block(code: QueryChain) -> Block {
    Block(code)
}

/// A code block. Surrounds the code with curly braces.
pub struct Block(QueryChain);

impl Buildable for Block {
    fn build(&self) -> String {
        format!("{{\n{}\n}}", self.0.build())
    }
}

impl From<QueryChain> for Block {
    fn from(code: QueryChain) -> Self {
        Self(code)
    }
}

impl From<Block> for Valuex {
    fn from(block: Block) -> Self {
        Valuex {
            string: block.build(),
            bindings: block.get_bindings(),
            errors: block.get_errors(),
        }
    }
}

impl Parametric for Block {
    fn get_bindings(&self) -> crate::BindingsList {
        self.0.get_bindings()
    }
}

impl Erroneous for Block {
    fn get_errors(&self) -> crate::ErrorList {
        self.0.get_errors()
    }
}

pub struct Bracket(Valuex);
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
