DEFINE FIELD labels ON planet TYPE array;
UPDATE planet SET labels = tags;
REMOVE FIELD tags ON TABLE planet;
