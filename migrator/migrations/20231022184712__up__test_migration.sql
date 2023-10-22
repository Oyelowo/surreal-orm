DEFINE EVENT event2 ON animal WHEN species = 'Homo Sapien' AND velocity < 10 THEN (SELECT * FROM eats);
DEFINE EVENT event1 ON animal WHEN species = 'Homo Erectus' AND velocity > 545 THEN (SELECT * FROM crop);
DEFINE INDEX species_speed_idx ON animal FIELDS species, velocity UNIQUE;
DEFINE FIELD velocity ON animal TYPE int;

REMOVE FIELD speed ON TABLE animal;