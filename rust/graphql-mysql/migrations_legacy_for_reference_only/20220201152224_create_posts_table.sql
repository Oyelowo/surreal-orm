-- Add migration script here
-- Add migration script here

create table if not exists posts
(
    id          uuid primary key default uuid_generate_v1mc(),
    user_id     uuid not null,
    created_at  timestamptz default timezone('utc', now()) not null,
    updated_at  timestamptz,
    deleted_at  timestamptz default null,
    title       text not null,
    content     text not null
);

SELECT trigger_updated_at('"posts"');

/* 
     // CONSTRAINT CREATION APPROACH THAT SHOULD BE USED EVERYWHERE FOR CONSISTENCY

This is the approach that should be followed throughout this codebase for creating FOREIGN key CONSTRAINT,
for the sake of consistency. 

There are other approaches for creating foreign keys in MySql. It's a matter of 
taste and flexibility. However, I chose this approach because:

- Separation of concern: it separates the key constraint creation from the main table fields creation

- gives the ability to create multiple(composite?) columns foreign key constraint e.g foreign key (a,b) references foo (x,y)
or foreign key (poster_id,group_id) references user_group (user_id, group_id)
This is not possible with inline foreign key constraint creation

- it's easy to just copy the alter table part below for new migration if I want to add new
foreign keys.
 */

alter table posts
    add constraint fk_posts_users
    foreign key (user_id)
    references users (id)