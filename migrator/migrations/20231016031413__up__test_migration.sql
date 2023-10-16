DEFINE TABLE eats SCHEMALESS;
DEFINE FIELD in ON eats TYPE record;
DEFINE FIELD place ON eats TYPE string;
DEFINE FIELD out ON eats TYPE record;
DEFINE FIELD id ON eats TYPE record<eats>;
DEFINE FIELD createdAt ON eats TYPE datetime;