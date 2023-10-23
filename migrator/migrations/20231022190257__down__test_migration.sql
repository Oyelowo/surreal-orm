DEFINE FIELD createdAt ON animal TYPE datetime;
UPDATE animal SET createdAt = updatedAt;
REMOVE FIELD updatedAt ON TABLE animal;

DEFINE FIELD err ON animal TYPE string;
UPDATE animal SET err = kingdom;
REMOVE FIELD kingdom ON TABLE animal;