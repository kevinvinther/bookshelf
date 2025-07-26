-- Add migration script here
create table users (
    id bigint auto_increment primary key,
    name varchar(255) not null,
    email varchar(255) not null unique,
    password varchar(255) not null,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);
