DEFINE FIELD kingdom ON animal TYPE string;
UPDATE animal SET kingdom = err;
REMOVE FIELD err ON TABLE animal;

DEFINE FIELD updatedAt ON animal TYPE datetime;
UPDATE animal SET updatedAt = createdAt;
REMOVE FIELD createdAt ON TABLE animal;