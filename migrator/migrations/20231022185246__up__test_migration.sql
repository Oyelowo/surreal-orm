DEFINE TABLE crop SCHEMAFULL;
DEFINE FIELD id ON crop TYPE record<crop>;

DEFINE FIELD color ON crop TYPE string;

DEFINE TABLE planet SCHEMAFULL;
DEFINE FIELD lastName ON planet TYPE string;

DEFINE FIELD id ON planet TYPE record<planet>;

DEFINE FIELD population ON planet TYPE int;

DEFINE FIELD tags ON planet TYPE array;

DEFINE FIELD created ON planet TYPE datetime;

DEFINE TABLE student SCHEMAFULL;
DEFINE FIELD age ON student TYPE int;

DEFINE FIELD id ON student TYPE record<student>;

DEFINE FIELD school ON student TYPE string;

DEFINE FIELD class ON student TYPE string;

DEFINE TABLE animal SCHEMAFULL;
DEFINE FIELD species ON animal TYPE string;

DEFINE FIELD characteristics ON animal TYPE array;

DEFINE FIELD createdAt ON animal TYPE datetime;

DEFINE FIELD id ON animal TYPE record<animal>;

DEFINE FIELD velocity ON animal TYPE int;

DEFINE FIELD err ON animal TYPE string;

DEFINE INDEX species_speed_idx ON animal FIELDS species, velocity UNIQUE;
DEFINE EVENT event2 ON animal WHEN species = 'Homo Sapien' AND velocity < 10 THEN (SELECT * FROM eats);
DEFINE EVENT event1 ON animal WHEN species = 'Homo Erectus' AND velocity > 545 THEN (SELECT * FROM crop);
DEFINE TABLE eats SCHEMAFULL;
DEFINE FIELD id ON eats TYPE record<eats>;

DEFINE FIELD in ON eats TYPE record;

DEFINE FIELD place ON eats TYPE string;

DEFINE FIELD out ON eats TYPE record;

DEFINE FIELD createdAt ON eats TYPE datetime;