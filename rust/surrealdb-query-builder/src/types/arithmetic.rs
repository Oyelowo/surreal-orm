// use std::ops::Add;
//
//
// impl Add

use std::ops::{Add, Div, Mul, Sub};

use crate::{statements::LetStatement, Buildable, Field, NumberLike, Operation, Parametric};

// impl Add for Field {
//     type Output = Operation;
//
//     fn add(self, rhs: Self) -> Self::Output {
//         self.add(rhs)
//     }
// }

impl<T: Into<NumberLike>> Add<T> for LetStatement {
    type Output = Operation;

    fn add(self, rhs: T) -> Self::Output {
        let rhs: NumberLike = rhs.into();
        Operation {
            query_string: format!("{} + {}", self.get_param().build(), rhs.build()),
            bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
            errors: vec![],
        }
    }
}

impl<T: Into<NumberLike>> Add<T> for &LetStatement {
    type Output = Operation;

    fn add(self, rhs: T) -> Self::Output {
        let rhs: NumberLike = rhs.into();
        Operation {
            query_string: format!("{} + {}", self.get_param().build(), rhs.build()),
            bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
            errors: vec![],
        }
    }
}
// impl Add<&LetStatement> for &LetStatement {
//     type Output = Operation;
//
//     fn add(self, rhs: &LetStatement) -> Self::Output {
//         // let rhs: NumberLike = rhs.into();
//         Operation {
//             query_string: format!("{} + {}", self.build(), rhs.build()),
//             bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
//             errors: vec![],
//         }
//     }
// }

impl<T: Into<NumberLike>> Sub<T> for &LetStatement {
    type Output = Operation;

    fn sub(self, rhs: T) -> Self::Output {
        let rhs: NumberLike = rhs.into();
        Operation {
            query_string: format!("{} - {}", self.get_param().build(), rhs.build()),
            bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
            errors: vec![],
        }
    }
}

impl<T: Into<NumberLike>> Mul<T> for &LetStatement {
    type Output = Operation;

    fn mul(self, rhs: T) -> Self::Output {
        let rhs: NumberLike = rhs.into();
        Operation {
            query_string: format!("{} * {}", self.get_param().build(), rhs.build()),
            bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
            errors: vec![],
        }
    }
}

impl<T: Into<NumberLike>> Div<T> for &LetStatement {
    type Output = Operation;

    fn div(self, rhs: T) -> Self::Output {
        let rhs: NumberLike = rhs.into();
        Operation {
            query_string: format!("{} / {}", self.get_param().build(), rhs.build()),
            bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
            errors: vec![],
        }
    }
}

impl<T: Into<NumberLike>> Add<T> for &Field {
    type Output = Operation;

    fn add(self, rhs: T) -> Self::Output {
        // self.add(rhs)
        let rhs: NumberLike = rhs.into();
        Operation {
            query_string: format!("{} + {}", self.build(), rhs.build()),
            bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
            errors: vec![],
        }
    }
}

impl<T: Into<NumberLike>> Add<T> for Field {
    type Output = Operation;

    fn add(self, rhs: T) -> Self::Output {
        // self.add(rhs)
        let rhs: NumberLike = rhs.into();
        Operation {
            query_string: format!("{} + {}", self.build(), rhs.build()),
            bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
            errors: vec![],
        }
    }
}

impl<T: Into<NumberLike>> Sub<T> for Field {
    type Output = Operation;

    fn sub(self, rhs: T) -> Self::Output {
        // self.add(rhs)
        let rhs: NumberLike = rhs.into();
        Operation {
            query_string: format!("{} - {}", self.build(), rhs.build()),
            bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
            errors: vec![],
        }
    }
}

impl<T: Into<NumberLike>> Mul<T> for Field {
    type Output = Operation;

    fn mul(self, rhs: T) -> Self::Output {
        // self.add(rhs)
        let rhs: NumberLike = rhs.into();
        Operation {
            query_string: format!("{} * {}", self.build(), rhs.build()),
            bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
            errors: vec![],
        }
    }
}

impl<T: Into<NumberLike>> Div<T> for Field {
    type Output = Operation;

    fn div(self, rhs: T) -> Self::Output {
        // self.add(rhs)
        let rhs: NumberLike = rhs.into();
        Operation {
            query_string: format!("{} / {}", self.build(), rhs.build()),
            bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
            errors: vec![],
        }
    }
}

// trait AddField<Rhs = Self> {
//     type Output;
//
//     fn add_field(self, rhs: Rhs) -> Self::Output;
// }
//
// impl<T: Into<Field>, U: Into<Field>> AddField<U> for T {
//     type Output = Operation;
//
//     fn add_field(self, rhs: U) -> Self::Output {
//         let lhs_field: Field = self.into();
//         let rhs_field: Field = rhs.into();
//         // Actual addition logic goes here
//         todo!()
//     }
// }

// impl Add<Field> for Operation {
//     type Output = Operation;
//
//     fn add(self, rhs: Field) -> Self::Output {
//         self.add(rhs)
//     }
// }

impl<T: Into<NumberLike>> Add<T> for Operation {
    type Output = Operation;

    fn add(self, rhs: T) -> Self::Output {
        let rhs: NumberLike = rhs.into();
        Operation {
            query_string: format!("{} + {}", self.build(), rhs.build()),
            bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
            errors: vec![],
        }
    }
}

impl<T: Into<NumberLike>> Sub<T> for Operation {
    type Output = Operation;

    fn sub(self, rhs: T) -> Self::Output {
        let rhs: NumberLike = rhs.into();
        Operation {
            query_string: format!("{} - {}", self.build(), rhs.build()),
            bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
            errors: vec![],
        }
    }
}

impl<T: Into<NumberLike>> Div<T> for Operation {
    type Output = Operation;

    fn div(self, rhs: T) -> Self::Output {
        let rhs: NumberLike = rhs.into();
        Operation {
            query_string: format!("{} / {}", self.build(), rhs.build()),
            bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
            errors: vec![],
        }
    }
}

impl<T: Into<NumberLike>> Mul<T> for Operation {
    type Output = Operation;

    fn mul(self, rhs: T) -> Self::Output {
        let rhs: NumberLike = rhs.into();
        Operation {
            query_string: format!("{} * {}", self.build(), rhs.build()),
            bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
            errors: vec![],
        }
    }
}

impl Add<Operation> for Operation {
    type Output = Operation;

    fn add(self, rhs: Operation) -> Self::Output {
        Operation {
            query_string: format!("({}) + ({})", self.build(), rhs.build()),
            bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
            errors: vec![],
        }
    }
}

impl Sub<Operation> for Operation {
    type Output = Operation;

    fn sub(self, rhs: Operation) -> Self::Output {
        Operation {
            query_string: format!("({}) - ({})", self.build(), rhs.build()),
            bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
            errors: vec![],
        }
    }
}

impl Div<Operation> for Operation {
    type Output = Operation;

    fn div(self, rhs: Operation) -> Self::Output {
        Operation {
            query_string: format!("({}) / ({})", self.build(), rhs.build()),
            bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
            errors: vec![],
        }
    }
}

impl Mul<Operation> for Operation {
    type Output = Operation;

    fn mul(self, rhs: Operation) -> Self::Output {
        Operation {
            query_string: format!("({}) * ({})", self.build(), rhs.build()),
            bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
            errors: vec![],
        }
    }
}
// impl<T: Into<NumberLike>> Add for T {
//     type Output = NumberLike;
//
//     fn add(self, other: Self) -> Self::Output {
//         let lhs: NumberLike = self.into();
//         let rhs: NumberLike = other.into();
//
//         let new_bindings = [lhs.0.bindings.clone(), rhs.0.bindings.clone()].concat();
//         let new_string = format!("{} + {}", lhs.0.string, rhs.0.string);
//
//         NumberLike(Valuex {
//             string: new_string,
//             bindings: new_bindings,
//             errors: vec![],
//         })
//     }
// }
