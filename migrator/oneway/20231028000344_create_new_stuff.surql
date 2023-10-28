DEFINE TABLE migration SCHEMAFULL;
DEFINE FIELD timestamp ON migration TYPE int;
DEFINE FIELD id ON migration TYPE record<migration>;
DEFINE FIELD name ON migration TYPE string;