/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::fmt;

use crate::{
    expression::Expression,
    traits::{BindingsList, Buildable, Erroneous, Parametric, Queryable},
    Block, ErrorList, Param,
};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum FlowType {
    InIterableData(Expression),
    OfIterableData(Expression),
}

#[derive(Debug, Clone)]
pub struct ForStatementData {
    item_params: Vec<Param>,
    flow_type: FlowType,
    block: Option<Block>,
    bindings: BindingsList,
    errors: ErrorList,
}


pub struct ForParam(Vec<Param>);

impl From<Vec<Param>> for ForParam {
    fn from(value: Vec<Param>) -> Self {
        Self(value)
    }
}

impl From<ForParam> for Vec<Param> {
    fn from(value: ForParam) -> Self {
        value.0
    }
}

impl From<&[Param]> for ForParam {
    fn from(value: &[Param]) -> Self {
        Self(value.to_vec())
    }
}

impl From<Param> for ForParam {
    fn from(value: Param) -> Self {
        Self(vec![value])
    }
}

impl From<&Param> for ForParam {
    fn from(value: &Param) -> Self {
        Self(vec![value.clone()])
    }
}

/// A helper function to create a for loop
/// ```
/// use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, statements::for_};
/// 
/// let ref __name = Param::new("name");
/// let ref person_table = Table::from("person");
/// let ref user_name = Field::from("user_name");
/// let for_loop = for_(__name).in_(vec!["Oyelowo", "Oyedayo"]).block(block! {
///    LET nick_name = select(user_name).from_only(person_table).where_(user_name.eq(__name));
///    select(All).from(person_table).where_(user_name.eq(nick_name));
/// });
/// println!("{}", for_loop);
/// ```
pub fn for_(params: impl Into<ForParam>) -> ForIterable {
    ForIterable(ForStatementData {
        item_params: params.into().into(),
        flow_type: FlowType::InIterableData(Expression::from("")),
        block: None,
        bindings: vec![],
        errors: vec![],
    })
}

pub struct ForIterable(ForStatementData);

impl ForIterable {
    #[allow(dead_code)]
    pub fn in_(self, iterable: impl Into<Expression>) -> ForStatementBlock {
        let mut data = self.0;
        data.flow_type = FlowType::InIterableData(iterable.into());
        // data.bindings.extend(data.iterable.get_bindings());
        // data.errors.extend(data.iterable.get_errors());
        ForStatementBlock(data)
    }

    // pub fn of_(self, iterable: impl Into<Expression>) -> ForStatementBlock {
    //     let mut data = self.0;
    //     data.flow_type = FlowType::OfIterableData(iterable.into());
    //     // data.bindings.extend(data.iterable.get_bindings());
    //     // data.errors.extend(data.iterable.get_errors());
    //     ForStatementBlock(data)
    // }
}

pub struct ForStatementBlock(ForStatementData);

pub struct ForStatement(ForStatementData);

impl ForStatementBlock {
    #[allow(dead_code)]
    pub fn block(mut self, block: impl Into<Block>) -> ForStatement {
        self.0.block = Some(block.into());
        ForStatement(self.0)
    }
}

impl Buildable for ForStatement {
    fn build(&self) -> String {
        let mut query = String::new();
        query.push_str("FOR ");
        query.push_str(
            &self
                .0
                .item_params
                .iter()
                .map(|x| x.build())
                .collect::<Vec<_>>()
                .join(", "),
        );

        query.push_str(" ");

        match &self.0.flow_type {
            FlowType::InIterableData(iterable) => {
                query.push_str("IN ");
                query.push_str(&iterable.build());
            }
            FlowType::OfIterableData(iterable) => {
                query.push_str("OF ");
                query.push_str(&iterable.build());
            }
        }
        query.push_str(" ");
        if let Some(block) = &self.0.block {
            query.push_str(&block.build());
        }
        query
    }
}

impl Parametric for ForStatement {
    fn get_bindings(&self) -> BindingsList {
        let mut bindings = self.0.bindings.to_vec();
        match &self.0.flow_type {
            FlowType::InIterableData(iterable) => {
                bindings.extend(iterable.get_bindings());
            }
            FlowType::OfIterableData(iterable) => {
                bindings.extend(iterable.get_bindings());
            }
        }
        if let Some(block) = &self.0.block {
            bindings.extend(block.get_bindings());
        }
        bindings
    }
}

impl Erroneous for ForStatement {
    fn get_errors(&self) -> ErrorList {
        let mut errors = self.0.errors.to_vec();
        match &self.0.flow_type {
            FlowType::InIterableData(iterable) => {
                errors.extend(iterable.get_errors());
            }
            FlowType::OfIterableData(iterable) => {
                errors.extend(iterable.get_errors());
            }
        }
        if let Some(block) = &self.0.block {
            errors.extend(block.get_errors());
        }
        errors
    }
}
impl Queryable for ForStatement {}

impl fmt::Display for ForStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{statements::select::{select, select_value}, *};

    #[test]
    fn test_for_in_block() {
        let ref __name = Param::new("name");
        let ref person_table = Table::from("person");
        let ref user_name = Field::from("user_name");

        let for_loop = for_(__name).in_(vec!["Oyelowo", "Oyedayo"]).block(block! {
            select(All).from(person_table).where_(user_name.eq(__name));
        });

        assert_eq!(
            for_loop.fine_tune_params(),
            "FOR $name IN $_param_00000001 {\nSELECT * FROM person WHERE user_name = $name;\n}"
        );
        assert_eq!(
            for_loop.to_raw().build(),
            "FOR $name IN ['Oyelowo', 'Oyedayo'] {\nSELECT * FROM person WHERE user_name = $name;\n}"
        );
    }

    #[test]
    fn test_for_in_with_block_macro() {
        let ref __name = Param::new("name");
        let ref person_table = Table::from("person");
        let ref user_name = Field::from("user_name");

        let for_loop = for_(__name).in_(vec!["Oyelowo", "Oyedayo"]).block(block! {
            LET nick_name = select(user_name).from_only(person_table).where_(user_name.eq(__name));


            select(All).from(person_table).where_(user_name.eq(nick_name));
        });

        assert_eq!(
            for_loop.fine_tune_params(),
            "FOR $name IN $_param_00000001 {\nLET $nick_name = $_param_00000002;\n\nSELECT * FROM person WHERE user_name = $nick_name;\n}"
        );

        assert_eq!(
            for_loop.to_raw().build(),
            "FOR $name IN ['Oyelowo', 'Oyedayo'] {\nLET $nick_name = (SELECT user_name FROM ONLY person WHERE user_name = $name);\n\nSELECT * FROM person WHERE user_name = $nick_name;\n}"
        );
    }

    #[test]
    fn test_for_in_block_with_subquery_iterable() {
        let ref __name = Param::new("name");
        let ref person_table = Table::from("person");
        let ref user_name = Field::from("user_name");

        let for_loop = for_(__name)
            .in_(
                select_value(user_name)
                    .from(person_table)
                    .where_(user_name.eq(__name)),
            )
            .block(block! {
                LET __nick_name = select(user_name).from_only(person_table).where_(user_name.eq(__name));
                
                select(All).from(person_table).where_(user_name.eq(__nick_name));
            });

        assert_eq!(
            for_loop.fine_tune_params(),
            "FOR $name IN $_param_00000001 {\nLET $__nick_name = $_param_00000002;\n\nSELECT * FROM person WHERE user_name = $__nick_name;\n}"
        );

        assert_eq!(
            for_loop.to_raw().build(),
            "FOR $name IN (SELECT VALUE user_name FROM person WHERE user_name = $name) {\nLET $__nick_name = (SELECT user_name FROM ONLY person WHERE user_name = $name);\n\nSELECT * FROM person WHERE user_name = $__nick_name;\n}"
        );
    }
}
