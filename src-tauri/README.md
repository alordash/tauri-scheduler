# Task scheduler

## Setup database
1. Declare postgres db url:
```
$ export DATABASE_URL="postgres://postgres:password@localhost/todos"
```
2. Create db
```
$ sqlx db create
```
3. Run migrations
```
$ sqlx migrate run
```
