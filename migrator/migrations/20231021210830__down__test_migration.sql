DEFINE FIELD firstName ON planet TYPE string;
UPDATE planet SET firstName = lastName;
REMOVE FIELD lastName ON TABLE planet;
REMOVE EVENT event1 ON TABLE animal;
REMOVE EVENT event2 ON TABLE animal;
REMOVE INDEX species_speed_idx ON TABLE animal;
REMOVE FIELD speed ON TABLE animal;
DEFINE FIELD attributes ON animal TYPE array;
UPDATE animal SET attributes = characteristics;
REMOVE FIELD characteristics ON TABLE animal;
DEFINE FIELD perre ON animal TYPE string;
REMOVE FIELD color ON TABLE crop;
DEFINE FIELD colour ON crop TYPE string;