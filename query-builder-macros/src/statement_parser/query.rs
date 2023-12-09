/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::fmt::Display;

use syn::{
    parse::{Parse, ParseBuffer, ParseStream},
    Expr, Ident, Result as SynResult, Token,
};

use super::{
    for_::ForLoopStatementParser,
    helpers::generate_variable_name,
    if_else::IfElseStatementAst,
    let_::LetStatementParser,
    return_::{ReturnExpr, ReturnStatementParser},
    transaction::{
        BeginTransactionStatementParser, CancelTransactionStatementParser,
        CommitTransactionStatementParser,
    },
};

#[derive(Debug, Clone)]
pub(crate) enum QueryParser {
    LetStatement(LetStatementParser),
    ForLoop(ForLoopStatementParser),
    IfEsle(IfElseStatementAst),
    BeginTransaction,
    CommitTransaction,
    CancelTransaction,
    ReturnStatement(ReturnStatementParser),
    BreakStatement,
    ContinueStatement,
    Expr { generated_ident: Ident, expr: Expr },
}

impl QueryParser {
    pub fn is_transaction_stmt(&self) -> bool {
        self.is_begin_transaction() || self.is_commit_transaction() || self.is_cancel_transaction()
    }

    pub fn is_begin_transaction(&self) -> bool {
        matches!(self, QueryParser::BeginTransaction)
    }

    pub fn is_commit_transaction(&self) -> bool {
        matches!(self, QueryParser::CommitTransaction)
    }

    pub fn is_cancel_transaction(&self) -> bool {
        matches!(self, QueryParser::CancelTransaction)
    }

    pub fn is_return_statement(&self) -> bool {
        match self {
            QueryParser::ReturnStatement(_) => true,
            // TODO: Consider removing IfElse and ForLoop from here.
            // We shouldn't use the following because the return statement from within
            // them do not blow up to the outer scope. They only return for the specific if else or
            // for loop block/context.
            // QueryParser::IfEsle(s) => s.has_return_statement(),
            // QueryParser::ForLoop(s) => s.has_return_statement(),
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq)]
enum TransactionType {
    Begin,
    Commit,
    Cancel,
    Invalid,
}

impl Display for TransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TransactionType::Begin => "begin transaction",
                TransactionType::Commit => "commit transaction",
                TransactionType::Cancel => "cancel transaction",
                TransactionType::Invalid => unreachable!(),
            }
        )
    }
}

impl TransactionType {
    pub fn is_transaction(input: &ParseBuffer<'_>) -> TransactionType {
        let input_str = input
            .to_string()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
            .to_lowercase();

        let likely_transaction = input.peek(Ident) && input.peek2(Ident) && input.peek3(Token![;]);

        if likely_transaction {
            if input_str.starts_with("begin transaction") {
                TransactionType::Begin
            } else if input_str.starts_with("commit transaction") {
                TransactionType::Commit
            } else if input_str.starts_with("cancel transaction") {
                TransactionType::Cancel
            } else {
                TransactionType::Invalid
            }
        } else {
            TransactionType::Invalid
        }
    }

    pub fn is_begin_transaction(input: &ParseBuffer<'_>) -> bool {
        Self::is_transaction(input) == Self::Begin
    }

    pub fn is_commit_transaction(input: &ParseBuffer<'_>) -> bool {
        Self::is_transaction(input) == Self::Commit
    }

    pub fn is_cancel_transaction(input: &ParseBuffer<'_>) -> bool {
        Self::is_transaction(input) == Self::Cancel
    }
}

enum StatementType {
    Let,
    Expr,
    Return,
    IfElse,
    ForLoop,
    Break,
    Continue,
    BeginTransaction,
    CommitTransaction,
    CancelTransaction,
}

impl<'a> From<&ParseBuffer<'a>> for StatementType {
    fn from(value: &ParseBuffer<'a>) -> Self {
        if value.peek(Token![let]) && value.peek2(Ident) && value.peek3(Token![=]) {
            StatementType::Let
        } else if value.peek(Token![return]) {
            StatementType::Return
        } else if value.peek(Token![for]) {
            StatementType::ForLoop
        } else if value.peek(Token![if]) {
            StatementType::IfElse
        } else if value.peek(Token![break]) && value.peek2(Token![;]) {
            StatementType::Break
        } else if value.peek(Token![continue]) && value.peek2(Token![;]) {
            StatementType::Continue
        } else if TransactionType::is_begin_transaction(value) {
            StatementType::BeginTransaction
        } else if TransactionType::is_commit_transaction(value) {
            StatementType::CommitTransaction
        } else if TransactionType::is_cancel_transaction(value) {
            StatementType::CancelTransaction
        } else {
            StatementType::Expr
        }
    }
}

impl Parse for QueryParser {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let statement_type = StatementType::from(input);

        match statement_type {
            StatementType::Let => {
                let let_statement = input.parse::<LetStatementParser>()?;
                Ok(QueryParser::LetStatement(let_statement))
            }
            StatementType::Expr => {
                let expr = input.parse::<Expr>()?;
                let _end: Token![;] = input.parse()?;
                Ok(QueryParser::Expr {
                    generated_ident: generate_variable_name(),
                    expr,
                })
            }
            StatementType::Return => {
                let _return: Token![return] = input.parse()?;
                let expr = input.parse::<ReturnExpr>()?;
                if input.peek(Token![;]) {
                    let _end: Token![;] = input.parse()?;
                }
                Ok(QueryParser::ReturnStatement(ReturnStatementParser {
                    _return,
                    expr,
                    // _end,
                    generated_ident: generate_variable_name(),
                }))
            }
            StatementType::Break => {
                let _break: Token![break] = input.parse()?;
                let _end: Token![;] = input.parse()?;
                Ok(QueryParser::BreakStatement)
            }
            StatementType::Continue => {
                let _continue: Token![continue] = input.parse()?;
                let _end: Token![;] = input.parse()?;
                Ok(QueryParser::ContinueStatement)
            }
            StatementType::BeginTransaction => {
                input.parse::<BeginTransactionStatementParser>()?;
                Ok(QueryParser::BeginTransaction)
            }
            StatementType::CommitTransaction => {
                input.parse::<CommitTransactionStatementParser>()?;
                Ok(QueryParser::CommitTransaction)
            }
            StatementType::CancelTransaction => {
                input.parse::<CancelTransactionStatementParser>()?;
                Ok(QueryParser::CancelTransaction)
            }
            StatementType::ForLoop => {
                let for_loop = input.parse::<ForLoopStatementParser>()?;
                Ok(QueryParser::ForLoop(for_loop))
            }
            StatementType::IfElse => {
                let if_else = input.parse::<IfElseStatementAst>()?;
                Ok(QueryParser::IfEsle(if_else))
            }
        }
    }
}
