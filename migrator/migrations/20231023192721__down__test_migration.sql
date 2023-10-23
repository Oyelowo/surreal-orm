DEFINE FIELD class ON student TYPE string;
UPDATE student SET class = room;
REMOVE FIELD room ON TABLE student;