DEFINE FIELD tags ON planet TYPE array;
UPDATE planet SET tags = labels;
REMOVE FIELD labels ON TABLE planet;