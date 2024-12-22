CREATE TABLE "games" (
    "id" UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    "word" TEXT NOT NULL,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT now (),
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT now ()
);
