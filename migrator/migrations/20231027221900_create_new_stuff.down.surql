REMOVE FIELD updatedAt ON TABLE planet;


DEFINE FIELD highSchool ON student TYPE string;
UPDATE student SET highSchool = university;
REMOVE FIELD university ON TABLE student;


DEFINE TABLE migration SCHEMALESS PERMISSIONS NONE;
REMOVE FIELD createdAt ON TABLE animal;