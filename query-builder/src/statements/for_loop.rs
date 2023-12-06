/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
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

/// A helper function to create a for loop
pub struct ForLoopStatement(ForStatementData);

impl ForStatementBlock {
    #[allow(dead_code)]
    pub fn block(mut self, block: impl Into<Block>) -> ForLoopStatement {
        self.0.block = Some(block.into());
        ForLoopStatement(self.0)
    }
}

impl Buildable for ForLoopStatement {
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

        query.push(' ');

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
        query.push(' ');
        if let Some(block) = &self.0.block {
            query.push_str(&block.build());
        }
        query.push(';');
        query
    }
}

impl Parametric for ForLoopStatement {
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

impl Erroneous for ForLoopStatement {
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
impl Queryable for ForLoopStatement {}

impl fmt::Display for ForLoopStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

#[cfg(test)]
mod tests {}
