DEFINE FIELD university ON student TYPE string;
UPDATE student SET university = school;
REMOVE FIELD school ON TABLE student;