DEFINE EVENT event2 ON animal WHEN species = 'Homo Sapien' AND speed < 10 THEN (SELECT * FROM eats);
DEFINE EVENT event1 ON animal WHEN species = 'Homo Erectus' AND speed > 545 THEN (SELECT * FROM crop);
DEFINE INDEX species_speed_idx ON animal FIELDS species, speed UNIQUE;
REMOVE FIELD velocity ON TABLE animal;

DEFINE FIELD speed ON animal TYPE int;