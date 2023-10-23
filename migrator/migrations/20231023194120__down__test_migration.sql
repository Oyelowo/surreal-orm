DEFINE FIELD attributes ON animal TYPE array;
UPDATE animal SET attributes = characteristics;
REMOVE FIELD characteristics ON TABLE animal;

DEFINE FIELD room ON student TYPE string;
UPDATE student SET room = class;
REMOVE FIELD class ON TABLE student;