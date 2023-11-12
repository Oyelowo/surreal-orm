/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::ops::{Add, Div, Mul, Sub};

use crate::{statements::LetStatement, Buildable, Field, NumberLike, Operation, Parametric};

impl<T: Into<NumberLike>> Add<T> for LetStatement {
    type Output = Operation;

    fn add(self, rhs: T) -> Self::Output {
        let mut rhs: NumberLike = rhs.into();
        Operation {
            query_string: format!(
                "{} + {}",
                self.get_param().build(),
                rhs.bracket_if_operation().build()
            ),
            bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
            errors: vec![],
        }
    }
}

impl<T: Into<NumberLike>> Add<T> for &LetStatement {
    type Output = Operation;

    fn add(self, rhs: T) -> Self::Output {
        let mut rhs: NumberLike = rhs.into();
        Operation {
            query_string: format!(
                "{} + {}",
                self.get_param().build(),
                rhs.bracket_if_operation().build()
            ),
            bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
            errors: vec![],
        }
    }
}

impl<T: Into<NumberLike>> Sub<T> for &LetStatement {
    type Output = Operation;

    fn sub(self, rhs: T) -> Self::Output {
        let mut rhs: NumberLike = rhs.into();
        Operation {
            query_string: format!(
                "{} - {}",
                self.get_param().build(),
                rhs.bracket_if_operation().build()
            ),
            bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
            errors: vec![],
        }
    }
}

impl<T: Into<NumberLike>> Mul<T> for LetStatement {
    type Output = Operation;

    fn mul(self, rhs: T) -> Self::Output {
        let mut rhs: NumberLike = rhs.into();
        Operation {
            query_string: format!(
                "{} * {}",
                self.get_param().build(),
                rhs.bracket_if_operation().build()
            ),
            bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
            errors: vec![],
        }
    }
}

impl<T: Into<NumberLike>> Mul<T> for &LetStatement {
    type Output = Operation;

    fn mul(self, rhs: T) -> Self::Output {
        let mut rhs: NumberLike = rhs.into();
        Operation {
            query_string: format!(
                "{} * {}",
                self.get_param().build(),
                rhs.bracket_if_operation().build()
            ),
            bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
            errors: vec![],
        }
    }
}

impl<T: Into<NumberLike>> Div<T> for LetStatement {
    type Output = Operation;

    fn div(self, rhs: T) -> Self::Output {
        let mut rhs: NumberLike = rhs.into();
        Operation {
            query_string: format!(
                "{} / {}",
                self.get_param().build(),
                rhs.bracket_if_operation().build()
            ),
            bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
            errors: vec![],
        }
    }
}

impl<T: Into<NumberLike>> Div<T> for &LetStatement {
    type Output = Operation;

    fn div(self, rhs: T) -> Self::Output {
        let mut rhs: NumberLike = rhs.into();
        Operation {
            query_string: format!(
                "{} / {}",
                self.get_param().build(),
                rhs.bracket_if_operation().build()
            ),
            bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
            errors: vec![],
        }
    }
}

impl<T: Into<NumberLike>> Add<T> for &Field {
    type Output = Operation;

    fn add(self, rhs: T) -> Self::Output {
        let mut rhs: NumberLike = rhs.into();
        Operation {
            query_string: format!("{} + {}", self.build(), rhs.bracket_if_operation().build()),
            bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
            errors: vec![],
        }
    }
}

impl<T: Into<NumberLike>> Add<T> for Field {
    type Output = Operation;

    fn add(self, rhs: T) -> Self::Output {
        let mut rhs: NumberLike = rhs.into();
        Operation {
            query_string: format!("{} + {}", self.build(), rhs.bracket_if_operation().build()),
            bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
            errors: vec![],
        }
    }
}

impl<T: Into<NumberLike>> Sub<T> for Field {
    type Output = Operation;

    fn sub(self, rhs: T) -> Self::Output {
        let mut rhs: NumberLike = rhs.into();
        Operation {
            query_string: format!("{} - {}", self.build(), rhs.bracket_if_operation().build()),
            bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
            errors: vec![],
        }
    }
}

impl<T: Into<NumberLike>> Mul<T> for Field {
    type Output = Operation;

    fn mul(self, rhs: T) -> Self::Output {
        let mut rhs: NumberLike = rhs.into();
        Operation {
            query_string: format!("{} * {}", self.build(), rhs.bracket_if_operation().build()),
            bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
            errors: vec![],
        }
    }
}

impl<T: Into<NumberLike>> Div<T> for Field {
    type Output = Operation;

    fn div(self, rhs: T) -> Self::Output {
        let mut rhs: NumberLike = rhs.into();
        Operation {
            query_string: format!("{} / {}", self.build(), rhs.bracket_if_operation().build()),
            bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
            errors: vec![],
        }
    }
}

impl<T: Into<NumberLike>> Add<T> for Operation {
    type Output = Operation;

    fn add(self, rhs: T) -> Self::Output {
        let mut rhs: NumberLike = rhs.into();
        Operation {
            query_string: format!(
                "({}) + {}",
                self.build(),
                rhs.bracket_if_operation().build()
            ),
            bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
            errors: vec![],
        }
    }
}

impl<T: Into<NumberLike>> Sub<T> for Operation {
    type Output = Operation;

    fn sub(self, rhs: T) -> Self::Output {
        let mut rhs: NumberLike = rhs.into();
        Operation {
            query_string: format!(
                "({}) - {}",
                self.build(),
                rhs.bracket_if_operation().build()
            ),
            bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
            errors: vec![],
        }
    }
}

impl<T: Into<NumberLike>> Div<T> for Operation {
    type Output = Operation;

    fn div(self, rhs: T) -> Self::Output {
        let mut rhs: NumberLike = rhs.into();
        Operation {
            query_string: format!(
                "({}) / {}",
                self.build(),
                rhs.bracket_if_operation().build()
            ),
            bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
            errors: vec![],
        }
    }
}

impl<T: Into<NumberLike>> Mul<T> for Operation {
    type Output = Operation;

    fn mul(self, rhs: T) -> Self::Output {
        let mut rhs: NumberLike = rhs.into();
        Operation {
            query_string: format!(
                "({}) * {}",
                self.build(),
                rhs.bracket_if_operation().build()
            ),
            bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
            errors: vec![],
        }
    }
}
