DEFINE FIELD createdAt ON animal TYPE datetime;
DEFINE FIELD terr ON animal TYPE string;
REMOVE FIELD place ON TABLE eats;
REMOVE FIELD createdAt ON TABLE eats;
DEFINE FIELD color ON crop TYPE string;