DEFINE INDEX species_speed_idx ON animal FIELDS species, velocity UNIQUE;
DEFINE EVENT event2 ON animal WHEN (species = 'Homo Sapien') AND (velocity < 10) THEN (SELECT * FROM eats);