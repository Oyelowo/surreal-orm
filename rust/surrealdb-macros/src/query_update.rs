struct UpdateQuery<'a> {
    target: &'a str,
    content: Option<&'a str>,
    merge: Option<&'a str>,
    set: Vec<(&'a str, &'a str)>,
    condition: Option<&'a str>,
    return_type: ReturnType,
    timeout: Option<&'a str>,
    parallel: bool,
}

enum ReturnType {
    None,
    Before,
    After,
    Diff,
    Fields(Vec<&'static str>),
}

impl<'a> UpdateQuery<'a> {
    fn new(target: &'a str) -> Self {
        Self {
            target,
            content: None,
            merge: None,
            set: Vec::new(),
            condition: None,
            return_type: ReturnType::After,
            timeout: None,
            parallel: false,
        }
    }

    fn content(mut self, content: &'a str) -> Self {
        self.content = Some(content);
        self
    }

    fn merge(mut self, merge: &'a str) -> Self {
        self.merge = Some(merge);
        self
    }

    fn set(mut self, field: &'a str, value: &'a str) -> Self {
        self.set.push((field, value));
        self
    }

    fn condition(mut self, condition: &'a str) -> Self {
        self.condition = Some(condition);
        self
    }

    fn return_type(mut self, return_type: ReturnType) -> Self {
        self.return_type = return_type;
        self
    }

    fn timeout(mut self, timeout: &'a str) -> Self {
        self.timeout = Some(timeout);
        self
    }

    fn parallel(mut self) -> Self {
        self.parallel = true;
        self
    }

    fn build(&self) -> String {
        let mut query = String::new();

        query.push_str("UPDATE ");
        query.push_str(self.target);

        if let Some(content) = self.content {
            query.push_str(" CONTENT ");
            query.push_str(content);
        } else if let Some(merge) = self.merge {
            query.push_str(" MERGE ");
            query.push_str(merge);
        } else if !self.set.is_empty() {
            query.push_str(" SET ");
            for (i, (field, value)) in self.set.iter().enumerate() {
                if i > 0 {
                    query.push_str(", ");
                }
                query.push_str(field);
                query.push_str(" = ");
                query.push_str(value);
            }
        }

        if let Some(condition) = self.condition {
            query.push_str(" WHERE ");
            query.push_str(condition);
        }

        match &self.return_type {
            ReturnType::None => query.push_str(" RETURN NONE"),
            ReturnType::Before => query.push_str(" RETURN BEFORE"),
            ReturnType::After => {} // Default, do nothing
            ReturnType::Diff => query.push_str(" RETURN DIFF"),
            ReturnType::Fields(fields) => {
                query.push_str(" RETURN ");
                for (i, field) in fields.iter().enumerate() {
                    if i > 0 {
                        query.push_str(", ");
                    }
                    query.push_str(field);
                }
            }
        }

        if let Some(timeout) = self.timeout {
            query.push_str(" TIMEOUT ");
            query.push_str(timeout);
        }

        if self.parallel {
            query.push_str(" PARALLEL");
        }

        query
    }
}
