-- Your SQL goes here


CREATE TABLE "rig_components"(
	"rig_id" UUID NOT NULL REFERENCES rigs(id),
	"component_id" UUID NOT NULL REFERENCES components(id),
	PRIMARY KEY("rig_id", "component_id")
);

