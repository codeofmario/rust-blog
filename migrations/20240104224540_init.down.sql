-- ### COMMENTS
drop trigger if exists set_timestamp_comments on comments;
drop table if exists comments;

-- ### POSTS
drop trigger if exists set_timestamp_posts on posts;
drop table if exists posts;

-- ### USERS
drop trigger if exists set_timestamp_users on users;
drop table if exists users;

-- ### Function
drop function if exists trigger_set_timestamp();