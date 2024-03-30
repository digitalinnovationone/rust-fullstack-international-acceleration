-- Your SQL goes here
CREATE TABLE "components"(
	"id" UUID NOT NULL PRIMARY KEY,
	"manufacturer" VARCHAR NOT NULL,
	"model" VARCHAR NOT NULL,
	"slot" SLOT NOT NULL,
	"price" INT4 NOT NULL
);

