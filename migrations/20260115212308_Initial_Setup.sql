-- Add migration script here
create table if not exists users (
    id  integer primary key,
    username text not null
);


insert into users (username)
values ('Ryan Way'), ('John Smith'), ('Jane Doe')