DEFINE FIELD updatedAt ON planet TYPE datetime;
DEFINE FIELD university ON student TYPE string;
UPDATE student SET university = highSchool;
REMOVE FIELD highSchool ON TABLE student;
REMOVE TABLE migration;
DEFINE FIELD createdAt ON animal TYPE datetime;