# Model

In Surreal, a Model represents a blueprint of your data model consisting of
various Nodes and Edges. A Model is a collection of various Nodes (entities) and
their relationships (Edges), providing a comprehensive view of your data.

The Object struct is used to define a Model, and it has its own set of struct
and field attributes. For instance, the `rename_all` struct attribute lets you
define a case convention for all the fields in the Model. And the `rename` field
attribute allows you to specify a different name for a field.
