use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct DbField {
    field_name: String,
}

impl DbField {
    pub fn new(field_name: &str) -> Self {
        Self {
            field_name: field_name.to_owned(),
        }
    }

    pub fn get_field_name(&self) -> &str {
        &self.field_name
    }

    pub fn equals<T: Display>(&self, value: T) -> DbQuery {
        DbQuery::new(format!("{} = {}", self.field_name, value))
    }

    pub fn not_equals<T: Display>(&self, value: T) -> DbQuery {
        DbQuery::new(format!("{} != {}", self.field_name, value))
    }

    pub fn greater_than<T: Display>(&self, value: T) -> DbQuery {
        DbQuery::new(format!("{} > {}", self.field_name, value))
    }

    pub fn greater_than_or_equals<T: Display>(&self, value: T) -> DbQuery {
        DbQuery::new(format!("{} >= {}", self.field_name, value))
    }

    pub fn less_than<T: Display>(&self, value: T) -> DbQuery {
        DbQuery::new(format!("{} < {}", self.field_name, value))
    }

    pub fn less_than_or_equals<T: Display>(&self, value: T) -> DbQuery {
        DbQuery::new(format!("{} <= {}", self.field_name, value))
    }

    pub fn between<T: Display, U: Display>(&self, lower_bound: T, upper_bound: U) -> DbQuery {
        DbQuery::new(format!(
            "{} BETWEEN {} AND {}",
            self.field_name, lower_bound, upper_bound
        ))
    }

    pub fn like(&self, pattern: &str) -> DbQuery {
        DbQuery::new(format!("{} LIKE '{}'", self.field_name, pattern))
    }

    pub fn not_like(&self, pattern: &str) -> DbQuery {
        DbQuery::new(format!("{} NOT LIKE '{}'", self.field_name, pattern))
    }

    pub fn is_null(&self) -> DbQuery {
        DbQuery::new(format!("{} IS NULL", self.field_name))
    }

    pub fn is_not_null(&self) -> DbQuery {
        DbQuery::new(format!("{} IS NOT NULL", self.field_name))
    }
}
