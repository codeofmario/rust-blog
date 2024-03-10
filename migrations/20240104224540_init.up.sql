-- ### update "update_at" field
create or replace function trigger_set_timestamp()
    returns trigger as
$$
begin
    new.updated_at = now();
    return new;
end;
$$ language plpgsql;

-- ### USERS
CREATE TABLE IF NOT EXISTS users
(
    id         uuid primary key     default gen_random_uuid(),
    email      varchar     not null,
    username   varchar     not null,
    password   varchar     not null,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

create trigger set_timestamp
    before update
    on users
    for each row
execute procedure trigger_set_timestamp();

-- ### POSTS
create table if not exists posts
(
    id         uuid primary key     default gen_random_uuid(),
    title      varchar     not null,
    body       text        not null,
    image_id   uuid        not null,
    user_id    uuid        not null references users (id) on delete cascade,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

create trigger set_timestamp
    before update
    on posts
    for each row
execute procedure trigger_set_timestamp();

-- ### COMMENTS
create table if not exists comments
(
    id         uuid primary key     default gen_random_uuid(),
    body       text        not null,
    user_id    uuid        not null references users (id) on delete cascade,
    post_id    uuid        not null references posts (id) on delete cascade,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

create trigger set_timestamp
    before update
    on comments
    for each row
execute procedure trigger_set_timestamp();
