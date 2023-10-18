DEFINE TABLE eats SCHEMAFULL;
DEFINE FIELD place ON eats TYPE string;
DEFINE FIELD out ON eats TYPE record;
DEFINE FIELD createdAt ON eats TYPE datetime;
DEFINE FIELD in ON eats TYPE record;
DEFINE FIELD id ON eats TYPE record<eats>;
DEFINE FIELD id ON animal TYPE record<animal>;
DEFINE FIELD speciesNamx ON animal TYPE string;
DEFINE FIELD attributes ON animal TYPE array;