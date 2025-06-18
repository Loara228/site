create table if not exists users (
    id serial primary key,
    username varchar(32),
    password varchar(64)
)