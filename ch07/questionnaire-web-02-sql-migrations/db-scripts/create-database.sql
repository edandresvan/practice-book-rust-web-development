BEGIN;

/* Drop tables if they already exist */
DROP TABLE IF EXISTS answers;
DROP TABLE IF EXISTS questions;

/* Create tables */
\i questions.sql;
\i answers.sql;



COMMIT;