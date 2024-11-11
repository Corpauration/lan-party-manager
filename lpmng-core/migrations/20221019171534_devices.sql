create table if not exists devices
(
    id          uuid    default gen_random_uuid() not null primary key,
    mac         text                              not null unique,
    user_id     uuid references users not null ,
    internet    boolean default false             not null,
    date_time   timestamp                         not null default now()
);