/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use crate::{
    table_events::ComparisonEvents, table_fields::ComparisonFields,
    table_indexes::ComparisonIndexes, *,
};
use serde::{Deserialize, Serialize};
use surreal_query_builder::{DbResources, Table};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TableResourcesData {
    events: Events,
    indexes: Indexes,
    tables: Tables,
    fields: Fields,
}

impl TableResourcesData {
    pub fn events(&self) -> Events {
        self.events.clone()
    }

    pub fn indexes(&self) -> Indexes {
        self.indexes.clone()
    }

    // This is usually empty in when getting the info from a table.
    // Only used when getting the info from a database.
    // So, turning it off for now
    // pub fn tables(&self) -> Tables {
    //     self.tables
    // }

    pub fn fields(&self) -> Fields {
        self.fields.clone()
    }
}
pub struct ComparisonTables<'a, R: DbResources> {
    pub resources: &'a ComparisonsInit<'a>,
    pub codebase_resources: &'a R,
    pub prompter: &'a dyn Prompter,
}

impl<R: DbResources> DbResourcesMeta<Tables> for ComparisonTables<'_, R> {
    fn get_left(&self) -> Tables {
        self.resources.left_resources.tables()
    }

    fn get_right(&self) -> Tables {
        self.resources.right_resources.tables()
    }

    fn queries(&self) -> MigrationResult<Queries> {
        let left = self.get_left().get_names_as_set();
        let right = self.get_right().get_names_as_set();
        let tables = left.union(&right);

        let mut queries = Queries::default();
        for table_name in tables {
            let initial_queries_len_state = queries.len();
            let def_left = self.get_left().get_definition(table_name).cloned();
            let def_right = self.get_right().get_definition(table_name).cloned();

            let events = ComparisonEvents {
                table: &Table::from(table_name.clone()),
                resources: self.resources,
            };

            let indexes = ComparisonIndexes {
                table: &Table::from(table_name.clone()),
                resources: self.resources,
            };

            let fields = ComparisonFields {
                table: &Table::from(table_name.clone()),
                resources: self.resources,
                codebase_resources: self.codebase_resources,
                prompter: self.prompter,
            };

            let fields = fields.queries()?;
            let indexes = indexes.queries()?;
            let events = events.queries()?;

            let extend_table_resources_up = |acc: &mut Queries| {
                acc.extend_up(&fields);
                acc.extend_up(&indexes);
                acc.extend_up(&events);
            };

            let extend_table_resources_down = |acc: &mut Queries| {
                acc.extend_down(&fields);
                acc.extend_down(&indexes);
                acc.extend_down(&events);
            };

            match DeltaTypeResource::from((def_left, def_right)) {
                DeltaTypeResource::NoChange => {
                    extend_table_resources_up(&mut queries);
                    extend_table_resources_down(&mut queries);
                }
                DeltaTypeResource::Update { left, right } => {
                    queries.add_up(QueryType::Define(right));
                    extend_table_resources_up(&mut queries);
                    extend_table_resources_down(&mut queries);

                    queries.add_down(QueryType::Define(left));
                }
                DeltaTypeResource::Create { right } => {
                    queries.add_down(QueryType::Remove(right.as_remove_statement()?));

                    queries.add_up(QueryType::Define(right));
                    extend_table_resources_up(&mut queries);
                }
                DeltaTypeResource::Remove { left } => {
                    queries.add_up(QueryType::Remove(left.as_remove_statement()?));
                    queries.add_down(QueryType::Define(left));
                    extend_table_resources_down(&mut queries);
                }
            };

            if initial_queries_len_state.has_changed(&mut queries) {
                queries.add_new_line();
            }
        }

        Ok(queries)
    }
}

pub trait TableResourcesMeta<T>
where
    T: Informational,
{
    // Left is from migration dir
    fn get_left(&self) -> T;
    // Right is from codebase
    fn get_right(&self) -> T;

    fn get_table(&self) -> &Table;

    fn queries(&self) -> MigrationResult<Queries> {
        let left = self.get_left().get_names_as_set();
        let right = self.get_right().get_names_as_set();
        let table_resources_names = right.union(&left);

        let mut queries = Queries::default();
        for name in table_resources_names {
            let def_right = self.get_right().get_definition(name).cloned();
            let def_left = self.get_left().get_definition(name).cloned();

            match DeltaTypeResource::from((def_left, def_right)) {
                DeltaTypeResource::Create { right } => {
                    queries.add_down(QueryType::Remove(right.as_remove_statement()?));
                    queries.add_up(QueryType::Define(right));
                }
                DeltaTypeResource::Remove { left } => {
                    queries.add_up(QueryType::Remove(left.as_remove_statement()?));
                    queries.add_down(QueryType::Define(left));
                }
                DeltaTypeResource::Update { left, right } => {
                    queries.add_up(QueryType::Define(right));
                    queries.add_down(QueryType::Define(left));
                }
                DeltaTypeResource::NoChange => {}
            };
        }

        Ok(queries)
    }
}
