create table if not exists answers (
    id text not null primary key,
    email text not null,
    question text not null,
    hexagram text not null,
    related text not null,
    created_at datetime default current_timestamp
)
