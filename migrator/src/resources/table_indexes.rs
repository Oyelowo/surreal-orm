/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

use surreal_query_builder::Table;

use crate::*;

#[derive(Debug)]
pub(crate) struct ComparisonIndexes<'a> {
    pub(crate) table: &'a Table,
    pub(crate) resources: &'a ComparisonsInit<'a>,
}

impl<'a> TableResourcesMeta<Indexes> for ComparisonIndexes<'a> {
    fn get_left(&self) -> Indexes {
        self.resources
            .left_resources
            .get_table_indexes(self.get_table())
            .unwrap_or_default()
    }

    fn get_right(&self) -> Indexes {
        self.resources
            .right_resources
            .get_table_indexes(self.get_table())
            .unwrap_or_default()
    }

    fn get_table(&self) -> &Table {
        self.table
    }
}
