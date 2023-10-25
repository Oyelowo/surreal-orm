DEFINE TABLE animal SCHEMAFULL;
DEFINE FIELD attributes ON animal TYPE array;
DEFINE FIELD updatedAt ON animal TYPE datetime;
DEFINE FIELD id ON animal TYPE record<animal>;
DEFINE FIELD species ON animal TYPE string;
DEFINE FIELD velocity ON animal TYPE int;
DEFINE INDEX species_speed_idx ON animal FIELDS species, velocity UNIQUE;
DEFINE EVENT event2 ON animal WHEN species = 'Homo Sapien' AND velocity < 10 THEN (SELECT * FROM eats);
DEFINE EVENT event1 ON animal WHEN species = 'Homo Erectus' AND velocity > 545 THEN (SELECT * FROM crop);