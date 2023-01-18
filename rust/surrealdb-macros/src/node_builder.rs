use std::fmt::Display;

pub trait ToNodeBuilder<T: Display = Self>: Display {
    fn quoted(&self) -> String {
        format!("\"{self}\"")
    }

    /// Draws the start of a relation `->node`
    ///
    /// # Example
    /// ```
    /// use surreal_simple_querybuilder::prelude::*;
    ///
    /// let s = "user".with("project");
    ///
    /// assert_eq!("user->project", s);
    /// ```
    fn with(&self, relation_or_node: &str) -> String {
        // write the arrow only if the first character is not a special character.
        // there are cases where the `node` string that was passed starts with
        // an arrow or a dot, in which case we do not want to push a new arrow
        // ourselves.
        if !relation_or_node.starts_with("->") && !relation_or_node.starts_with(".") {
            format!("{self}->{relation_or_node}")
        } else {
            format!("{self}{relation_or_node}")
        }
    }

    /// Draws the end of a relation `<-node`
    ///
    /// # Example
    /// ```
    /// use surreal_simple_querybuilder::prelude::*;
    ///
    /// let s = "user".from("project");
    ///
    /// assert_eq!("user<-project", s);
    /// ```
    fn from(&self, node: &str) -> String {
        format!("{self}<-{node}")
    }

    /// Take the current string and add in front of it the given label name as to
    /// make a string of the following format `LabelName:CurrentString`
    ///
    /// # Example
    /// ```
    /// use surreal_simple_querybuilder::prelude::*;
    ///
    /// let label = "John".as_named_label("Account");
    ///
    /// assert_eq!(label, "Account:John");
    /// ```
    fn as_named_label(&self, label_name: &str) -> String {
        format!("{label_name}:{self}")
    }

    /// # Example
    /// ```
    /// use surreal_simple_querybuilder::prelude::*;
    ///
    /// let s = "user".equals("John");
    ///
    /// // Note that it doesn't add quotes around strings
    /// assert_eq!("user = John", s);
    /// ```
    fn equals(&self, value: &str) -> String {
        format!("{self} = {value}")
    }

    /// Take the current string and add `= $current_string` after it
    ///
    /// # Example
    /// ```
    /// use surreal_simple_querybuilder::prelude::*;
    ///
    /// let s = "account".equals_parameterized();
    ///
    /// assert_eq!("account = $account", s);
    /// ```
    fn equals_parameterized(&self) -> String {
        format!("{self} = ${self}")
    }

    /// Take the current string and add `> value` after it
    ///
    /// # Example
    /// ```
    /// use surreal_simple_querybuilder::prelude::*;
    ///
    /// let s = "account".greater_than("5");
    ///
    /// assert_eq!("account > 5", s);
    /// ```
    fn greater_than(&self, value: &str) -> String {
        format!("{self} > {value}")
    }

    /// Take the current string and add `+= value` after it
    ///
    /// # Example
    /// ```
    /// use surreal_simple_querybuilder::prelude::*;
    ///
    /// let s = "friends".plus_equal("account:john");
    ///
    /// assert_eq!("friends += account:john", s);
    /// ```
    fn plus_equal(&self, value: &str) -> String {
        format!("{self} += {value}")
    }

    /// # Example
    /// ```
    /// use surreal_simple_querybuilder::prelude::*;
    ///
    /// let s = "account".contains_one("'c'");
    ///
    /// assert_eq!("account CONTAINS 'c'", s);
    /// ```
    fn contains_one(&self, value: &str) -> String {
        format!("{self} CONTAINS {value}")
    }

    /// # Example
    /// ```
    /// use surreal_simple_querybuilder::prelude::*;
    ///
    /// let s = "account".contains_not("'z'");
    ///
    /// assert_eq!("account CONTAINSNOT 'z'", s);
    /// ```
    fn contains_not(&self, value: &str) -> String {
        format!("{self} CONTAINSNOT {value}")
    }

    /// # Example
    /// ```
    /// use surreal_simple_querybuilder::prelude::*;
    ///
    /// let s = "account".contains_all("['a', 'c', 'u']");
    ///
    /// assert_eq!("account CONTAINSALL ['a', 'c', 'u']", s);
    /// ```
    fn contains_all(&self, values: &str) -> String {
        format!("{self} CONTAINSALL {values}")
    }

    /// # Example
    /// ```
    /// use surreal_simple_querybuilder::prelude::*;
    ///
    /// let s = "account".contains_any("['a', 'c', 'u']");
    ///
    /// assert_eq!("account CONTAINSANY ['a', 'c', 'u']", s);
    /// ```
    fn contains_any(&self, values: &str) -> String {
        format!("{self} CONTAINSANY {values}")
    }

    /// # Example
    /// ```
    /// use surreal_simple_querybuilder::prelude::*;
    ///
    /// let s = "account".contains_none("['z', 'd', 'f']");
    ///
    /// assert_eq!("account CONTAINSNONE ['z', 'd', 'f']", s);
    /// ```
    fn contains_none(&self, values: &str) -> String {
        format!("{self} CONTAINSNONE {values}")
    }

    /// Take the current string and add `as alias` after it
    ///
    /// # Example
    /// ```
    /// use surreal_simple_querybuilder::prelude::*;
    ///
    /// let s = "account->manage->project".as_alias("account_projects");
    ///
    /// assert_eq!("account->manage->project AS account_projects", s);
    /// ```
    fn as_alias(&self, alias: &str) -> String {
        format!("{self} AS {alias}")
    }

    /// Take the current string, extract the last segment if it is a nested property,
    /// then add parenthesis around it and add the supplied condition in them.
    ///
    /// # Example
    /// ```
    /// use surreal_simple_querybuilder::prelude::*;
    ///
    /// let path = "account->manage->project";
    /// let s = path.filter("name = 'a_cool_project'");
    ///
    /// assert_eq!("account->manage->(project WHERE name = 'a_cool_project')", s);
    /// ```
    ///
    fn filter(&self, condition: &str) -> String {
        // This is a default implementation, but since we need the original string
        // to iterate over the chars the function does two string allocations.
        let original = self.to_string();
        let original_size = original.len();

        // this yields the size of the last segment, until a non alphanumeric character
        // is found.
        let last_segment_size = original
            .chars()
            .rev()
            .take_while(|c| c.is_alphanumeric())
            .count();

        let left = &original[..original_size - last_segment_size];
        let right = &original[original_size - last_segment_size..];

        format!("{left}({right} WHERE {condition})")
    }

    /// write a comma at the end of the string and append `right` after it.
    ///
    /// # Example
    /// ```
    /// use surreal_simple_querybuilder::prelude::*;
    ///
    /// let select = "*".comma("<-manage<-User as authors");
    /// let query = format!("select {select} from Files");
    ///
    /// assert_eq!("select *, <-manage<-User as authors from Files", query);
    /// ```
    fn comma(&self, right: &str) -> String {
        format!("{self}, {right}")
    }

    /// write a `count()` around the current string so that it sits between the
    /// parenthesis.
    ///
    /// # Example
    /// ```
    /// use surreal_simple_querybuilder::prelude::*;
    ///
    /// let count = "id".count();
    /// let query = format!("select {count} from Files");
    ///
    /// assert_eq!("select count(id) from Files", query);
    /// ```
    fn count(&self) -> String {
        format!("count({self})")
    }

    /// Add the supplied `id` right after the current string in order to get the a
    /// new string in the following format `current:id`
    /// # Example
    /// ```
    /// use surreal_simple_querybuilder::prelude::*;
    ///
    /// let query = "Account".with_id("John");
    ///
    /// assert_eq!(query, "Account:John");
    /// ```
    fn with_id(&self, id: &str) -> String {
        format!("{self}:{id}")
    }
}

impl<'a> ToNodeBuilder for &'a str {
    fn filter(&self, condition: &str) -> String {
        // unlike the default implementation of this trait function, the &str impl
        // does only one allocation.
        let original_size = self.len();

        // this yields the size of the last segment, until a non alphanumeric character
        // is found.
        let last_segment_size = self
            .chars()
            .rev()
            .take_while(|c| c.is_alphanumeric())
            .count();

        let left = &self[..original_size - last_segment_size];
        let right = &self[original_size - last_segment_size..];

        format!("{left}({right} WHERE {condition})")
    }
}

pub trait NodeBuilder<T: Display = Self>: Display {
    /// Draws the start of a relation `->node`
    ///
    /// # Example
    /// ```
    /// use surreal_simple_querybuilder::prelude::*;
    ///
    /// let s = "user".with("project");
    ///
    /// assert_eq!("user->project", s);
    /// ```
    fn with(&mut self, relation_or_node: &str) -> &mut String;

    /// Allows you to pass a lambda that should mutate the current string when the
    /// passed `condition` is `true`. If `condition` is `false` then the `action`
    /// lambda is ignored and the string stays intact.
    ///
    /// # Example
    /// ```
    /// use surreal_simple_querybuilder::prelude::*;
    ///
    /// // demonstrate how the given closure is ignored if the condition is `false`
    /// let mut label = "John".as_named_label("User");
    /// let intact = &mut label
    ///   .if_then(false, |s| s.with("LOVES").with("User"))
    ///   .with("FRIEND")
    ///   .with("User");
    ///
    /// assert_eq!("User:John->FRIEND->User", *intact);
    ///
    /// // demonstrate how the given closure is executed if the condition is `true`
    /// let mut label = "John".as_named_label("User");
    /// let modified = &mut label
    ///   .if_then(true, |s| s.with("LOVES").with("User"))
    ///   .with("FRIEND")
    ///   .with("User");
    ///
    /// assert_eq!("User:John->LOVES->User->FRIEND->User", *modified);
    /// ```
    fn if_then(&mut self, condition: bool, action: fn(&mut Self) -> &mut Self) -> &mut String;

    /// Take the current string add add `> value` after it
    ///
    /// # Example
    /// ```
    /// use surreal_simple_querybuilder::prelude::*;
    ///
    /// let s = "account".greater_than("5");
    ///
    /// assert_eq!("account > 5", s);
    /// ```
    fn greater_than(&mut self, value: &str) -> &mut String;

    /// Take the current string and add `+= value` after it
    ///
    /// # Example
    /// ```
    /// use surreal_simple_querybuilder::prelude::*;
    ///
    /// let s = "friends".plus_equal("account:john");
    ///
    /// assert_eq!("friends += account:john", s);
    /// ```
    fn plus_equal(&mut self, value: &str) -> &mut String;
}

impl NodeBuilder for String {
    fn with(&mut self, node: &str) -> &mut String {
        // push the arrow only if the first character is not a special character.
        // there are cases where the `node` string that was passed starts with
        // an arrow or a dot, in which case we do not want to push a new arrow
        // ourselves.
        if !node.starts_with("->") && !node.starts_with(".") {
            self.push_str("->");
        }

        self.push_str(node);

        self
    }

    fn if_then(&mut self, condition: bool, action: fn(&mut Self) -> &mut Self) -> &mut String {
        match condition {
            true => action(self),
            false => self,
        }
    }

    fn greater_than(&mut self, value: &str) -> &mut String {
        self.push_str(" > ");
        self.push_str(value);

        self
    }

    fn plus_equal(&mut self, value: &str) -> &mut String {
        self.push_str(" += ");
        self.push_str(value);

        self
    }
}

impl ToNodeBuilder for String {}
