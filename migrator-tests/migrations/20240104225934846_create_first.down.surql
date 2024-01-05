DEFINE FIELD attributes ON animal TYPE array<string> PERMISSIONS FULL;
UPDATE animal SET attributes = characteristics;
REMOVE FIELD characteristics ON TABLE animal;
DEFINE FIELD createdAt ON animal TYPE datetime PERMISSIONS FULL;
DEFINE FIELD updatedAt ON animal TYPE datetime PERMISSIONS FULL;
DEFINE INDEX species_speed_idx ON animal FIELDS species, velocity UNIQUE;


DEFINE EVENT event1 ON animal WHEN (species = 'Homo Erectus') AND (velocity > 545) THEN (SELECT * FROM crop);
DEFINE EVENT event2 ON animal WHEN (species = 'Homo Sapien') AND (velocity < 10) THEN (SELECT * FROM eats);






REMOVE FIELD location ON TABLE eats;
DEFINE FIELD place ON eats TYPE string PERMISSIONS FULL;
REMOVE FIELD updatedAt ON TABLE eats;








REMOVE TABLE new_stuff;
DEFINE FIELD firstName ON planet TYPE string PERMISSIONS FULL;
UPDATE planet SET firstName = newName;
REMOVE FIELD newName ON TABLE planet;




DEFINE TABLE student SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD age ON student TYPE int PERMISSIONS FULL;
DEFINE FIELD createdAt ON student TYPE datetime PERMISSIONS FULL;
DEFINE FIELD id ON student TYPE record<student> PERMISSIONS FULL;
DEFINE FIELD university ON student TYPE string PERMISSIONS FULL;
DEFINE FIELD updatedAt ON student TYPE datetime PERMISSIONS FULL;