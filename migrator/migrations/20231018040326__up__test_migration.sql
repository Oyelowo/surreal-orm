REMOVE FIELD createdAt ON TABLE animal;
REMOVE FIELD terr ON TABLE animal;
DEFINE FIELD place ON eats TYPE string;
DEFINE FIELD createdAt ON eats TYPE datetime;
REMOVE FIELD color ON TABLE crop;