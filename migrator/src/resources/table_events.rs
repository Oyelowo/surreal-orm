/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use surreal_orm::Table;

use crate::*;

#[derive(Debug)]
pub struct ComparisonEvents<'a> {
    pub(crate) table: &'a Table,
    pub(crate) resources: &'a ComparisonsInit<'a>,
}

impl<'a> TableResourcesMeta<Events> for ComparisonEvents<'a> {
    fn get_left(&self) -> Events {
        self.resources
            .left_resources
            .get_table_events(self.get_table())
            .unwrap_or_default()
    }

    fn get_right(&self) -> Events {
        self.resources
            .right_resources
            .get_table_events(self.get_table())
            .unwrap_or_default()
    }

    fn get_table(&self) -> &Table {
        self.table
    }
}
