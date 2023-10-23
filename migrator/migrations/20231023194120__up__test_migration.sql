DEFINE FIELD characteristics ON animal TYPE array;
UPDATE animal SET characteristics = attributes;
REMOVE FIELD attributes ON TABLE animal;

DEFINE FIELD class ON student TYPE string;
UPDATE student SET class = room;
REMOVE FIELD room ON TABLE student;