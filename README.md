# Practice Book: Rust Web Development


This repository contains my practical exercises from the book ["Rust Web Development" by Bastian Gruber (Manning)](https://www.manning.com/books/rust-web-development).


![Book Cover](https://images.manning.com/360/480/resize/book/9/57fa437-06ef-4a02-8070-bc33e0800c87/Gruber-HI.png)

The original code repository is also located [here in GitHub](https://github.com/Rust-Web-Development/code).

## License

[MIT](https://choosealicense.com/licenses/mit/)


The files in this repository are my own practice following the book lessons. 

However, the original copyright belongs to Manning Books.

Gruber, Bastian. 2023. Rust Web Development. Manning Publications. ISBN: 
978-1617299001

## PostgreSQL Installation

From chapter 7, a PostgreSQL database is used as the datastore for the web service of questions and answers. To install PostgreSQL using a [Podman](https://podman.io/) container and Ubuntu 22.10 follow these steps:

### Install Podman

```bash
$ sudo apt-get install aptitude;
$ sudo aptitude update && sudo aptitude upgrade -y;
$ sudo aptitude install podman;
```

### Install PostgreSQL Clients

```bash

$ wget --quiet -O - https://www.postgresql.org/media/keys/ACCC4CF8.asc | sudo gpg --dearmor -o /usr/share/keyrings/postgresql.gpg;
$ sudo sh -c 'echo "deb [signed-by=/usr/share/keyrings/postgresql.gpg] http://apt.postgresql.org/pub/repos/apt $(lsb_release -cs)-pgdg main" > /etc/apt/sources.list.d/postgresql.list';

$ wget --quiet -O - https://www.pgadmin.org/static/packages_pgadmin_org.pub | sudo gpg --dearmor -o /usr/share/keyrings/packages-pgadmin-org.gpg;

$ sudo sh -c 'echo "deb [signed-by=/usr/share/keyrings/packages-pgadmin-org.gpg] https://ftp.postgresql.org/pub/pgadmin/pgadmin4/apt/$(lsb_release -cs) pgadmin4 main" > /etc/apt/sources.list.d/pgadmin4.list';
 
$ sudo aptitude update && sudo aptitude upgrade -y;

$ sudo aptitude install libpq-dev postgresql-client pgadmin4-desktop -y;
```

### Install a PostgreSQL Container

Go to the source code directory for chapter 7 in this repository and ensure the SQL initialization script file to create the database is present:

```bash
$ cd ch07;
$ ls ./docker-entrypoint-initdb.d;
  init-db-rustwebdev.sh
```

We will use the PostgreSQL container based on Debian Linux from the Docker Registry.

Create a disk volume (i.e. a directory) to persist the database we will maintain in the container.

```bash
$ podman volume create questionnaire_volume;
```

Create the PostgreSQL container with the previously created disk volume and the customizable parameters. The first time, we specify the `POSTGRES_PASSWORD` parameter. Also the subdirectory with the initializaton script is mounted as a disk volume.

```bash
$ podman run --interactive --publish 5432:5432 --volume questionnaire_volume:/var/lib/postgresql/data --volume ./docker-entrypoint-initdb.d:/docker-entrypoint-initdb.d --memory 500m --env POSTGRES_PASSWORD=myP4ssw0rd --name questionnaire docker.io/library/postgres:15-bullseye;
```

## Execute PostgreSQL Scripts

Up to section 7.5, you can execute the SQL script `create-database.sql` to create the data structures.

```bash
$ cd db-scripts;
$ psql --host=localhost --port=5432 --dbname=rustwebdev --username=firstdev --password --file=create-database.sql;

```

From section 7.6, you can work with SQL migrations. First, delete the existing data structures:

```bash
$ cd db-scripts;
$ psql --host=localhost --port=5432 --dbname=rustwebdev --username=firstdev --password --file=drop-database.sql;

```






