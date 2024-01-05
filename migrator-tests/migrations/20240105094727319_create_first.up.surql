DEFINE FIELD characteristicsx ON animal TYPE array<string> PERMISSIONS FULL;
UPDATE animal SET characteristicsx = characteristics;
REMOVE FIELD characteristics ON TABLE animal;