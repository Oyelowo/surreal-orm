/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::ops::Deref;

use crate::Param;

use paste::paste;

macro_rules! define_param {
    ($(#[$attr:meta])* => $value:expr) => {
        paste! {
            $(#[$attr])*
            pub struct [< Param $value >](Param);

            impl Deref for [< Param $value >] {
                type Target = Param;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }


            $(#[$attr])*
            pub fn $value() -> [< Param $value >] {
                [< Param $value >](Param::new(stringify!($value)))
            }
        }
    };
}
// SurrealDB employs a set of predefined variables.
// While these variables can be utilized within your queries,
// it's important to note that you cannot declare new parameters with any of the names listed below:

define_param!(
    /// $auth: Represents the currently authenticated scope user
    => auth
);

define_param!(
    /// $token: Represents values held inside the JWT token used for the current session
    => token
);

define_param!(
    /// $session: Represents values from the session functions as an object
    => session
);

define_param!(
    /// $before: Represents the value before a mutation on a field
    => before
);

define_param!(
    /// $after: Represents the value after a mutation on a field
    => after
);

define_param!(
    /// $value: Represents the value after a mutation on a field (identical to $after in the case of an event)
    => value
);

define_param!(
    /// $input: Represents the initially inputted value in a field definition, as the value clause could have modified the $value variable
    => input
);

define_param!(
    /// $parent: Represents the parent record in a subquery
    => parent
);

define_param!(
    /// $event: Represents the type of table event triggered on an event
    => event
);

define_param!(
    /// $this: Represents the current record in a query
    => this
);
