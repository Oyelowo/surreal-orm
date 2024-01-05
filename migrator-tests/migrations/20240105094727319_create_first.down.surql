DEFINE FIELD characteristics ON animal TYPE array<string> PERMISSIONS FULL;
UPDATE animal SET characteristics = characteristicsx;
REMOVE FIELD characteristicsx ON TABLE animal;