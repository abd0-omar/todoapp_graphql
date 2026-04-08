-- Create "todos" table
CREATE TABLE "public"."todos" (
  "id" uuid NOT NULL DEFAULT gen_random_uuid(),
  "title" character varying(255) NOT NULL,
  "description" text NULL,
  "tags" text[] NULL,
  "is_completed" boolean NULL DEFAULT false,
  "created_at" timestamptz NULL DEFAULT CURRENT_TIMESTAMP,
  "updated_at" timestamptz NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY ("id")
);
