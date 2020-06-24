CREATE TABLE IF NOT EXISTS "completions" (
	"id" SERIAL PRIMARY KEY,
	"user" TEXT NOT NULL,
	"challenge_id" TEXT NOT NULL,
	"completion_time" TIMESTAMP DEFAULT now()
);

