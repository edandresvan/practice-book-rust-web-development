

CREATE TABLE questions (
  id serial PRIMARY KEY,
  title varchar (255) NOT NULL,
  content TEXT NOT NULL,
  tags TEXT [],
  created_on TIMESTAMP NOT NULL DEFAULT now()
);