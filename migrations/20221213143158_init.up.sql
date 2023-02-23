-- Add up migration script here

-- CREATE SCHEMA  template;
CREATE TYPE user_role AS ENUM ('user','admin');


CREATE TABLE users
(
    id        uuid PRIMARY KEY            DEFAULT uuid_generate_v4(),
    username  varchar(64) UNIQUE NOT NULL,
    password  varchar(64)        NOT NULL,
    create_at timestamptz        NOT NULL,
    update_at timestamptz        NOT NULL,
    role      user_role          NOT NULL DEFAULT 'user'
);

insert into users (username, password, create_at, update_at, role)
values ('user', '123456', now(), now(), 'user'),
       ('admin', '123456', now(), now(), 'admin');
