DEFINE FIELD firstName ON planet TYPE string;
UPDATE planet SET firstName = lastName;
REMOVE FIELD lastName ON TABLE planet;