DEFINE FIELD characteristics ON animal TYPE array<string> PERMISSIONS FULL;
UPDATE animal SET characteristics = attributes;
REMOVE FIELD attributes ON TABLE animal;
REMOVE FIELD createdAt ON TABLE animal;
REMOVE FIELD updatedAt ON TABLE animal;
DEFINE INDEX species_speed_idx ON animal FIELDS velocity, characteristics UNIQUE;


DEFINE EVENT event1 ON animal WHEN species = 'Homo Habillis' AND velocity > 545 THEN (SELECT * FROM crop);
REMOVE EVENT event2 ON TABLE animal;






DEFINE FIELD location ON eats TYPE string PERMISSIONS FULL;
REMOVE FIELD place ON TABLE eats;
DEFINE FIELD updatedAt ON eats TYPE datetime PERMISSIONS FULL;








DEFINE TABLE new_stuff SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD createdAt ON new_stuff TYPE datetime PERMISSIONS FULL;
DEFINE FIELD firstName ON new_stuff TYPE string PERMISSIONS FULL;
DEFINE FIELD id ON new_stuff TYPE record<new_stuff> PERMISSIONS FULL;
DEFINE FIELD updatedAt ON new_stuff TYPE datetime PERMISSIONS FULL;




DEFINE FIELD newName ON planet TYPE string PERMISSIONS FULL;
UPDATE planet SET newName = firstName;
REMOVE FIELD firstName ON TABLE planet;




REMOVE TABLE student;