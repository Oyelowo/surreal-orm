DEFINE FIELD room ON student TYPE string;
UPDATE student SET room = class;
REMOVE FIELD class ON TABLE student;