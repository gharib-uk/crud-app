# CRUD APP

Simple web app that is wrapper for basic CRUD operations on a Relational DB to manage members of 
a system.

I use rocket and diesel as ORM and web app frameworks.

The project contains :
- DB migrations (using `` diesel `` lib)
- Authentication

```
install libsqlite3-dev on debian based linux distros

cargo install diesel_cli --no-default-features --features sqlite3

diesel setup --database-url ./db.sqlite3

diesel migration generate create_members

diesel migration run --database-url=db.sqlite3

diesel migration revert --database-url=db.sqlite3
```