CREATE TABLE answers (
  id serial PRIMARY KEY,
  content TEXT NOT NULL,
  created_on TIMESTAMP NOT NULL DEFAULT now(),
  corresponding_question integer NOT NULL
);

ALTER TABLE answers 
  ADD CONSTRAINT fk_corresponding_question FOREIGN KEY (corresponding_question)
  REFERENCES questions(id);