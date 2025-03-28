create table if not exists member (
    id uuid,
    nickname varchar(255) not null unique,
    email varchar(255) not null unique,
    password varchar(255) not null,
    role varchar(255) not null check (role in ('ADMIN','MEMBER')),
    is_activated boolean default false not null,
    created_at timestamp(6) default current_timestamp,
    updated_at timestamp(6) default current_timestamp,
    primary key (id)
);

create table if not exists article_category(
    id bigserial,
    name varchar(255) unique,
    created_at timestamp(6) default current_timestamp,
    updated_at timestamp(6) default current_timestamp,
    primary key (id)
);

create table if not exists article (
    id uuid,
    category_id bigint,
    member_id uuid,
    title varchar(255) not null,
    contents text not null,
    view_count bigint default 0,
    created_at timestamp(6) not null default CURRENT_TIMESTAMP,
    updated_at timestamp(6) default CURRENT_TIMESTAMP,
    primary key (id),
    constraint FK_article_table_member FOREIGN KEY(member_id) REFERENCES member(id) ON DELETE CASCADE,
    constraint FK_article_table_category FOREIGN KEY(category_id) REFERENCES article_category(id) ON DELETE SET NULL
);

create table if not exists article_comment(
    id bigserial,
    member_id uuid,
    article_id uuid,
    parent_id bigint,
    contents text,
    created_at timestamp(6) default CURRENT_TIMESTAMP,
    updated_at timestamp(6) default CURRENT_TIMESTAMP,
    primary key (id),
    constraint FK_article_comment_table_article foreign key(article_id) references article(id) on delete cascade,
    constraint FK_article_comment_table_parent_comment foreign key(parent_id) references article_comment(id) on delete set null,
    constraint FK_article_comment_table_member foreign key(member_id) references member(id) on delete cascade
);

create table if not exists oauth2_member(
    id uuid,
    provider varchar(255) not null,
    user_id varchar(255) not null,
    email varchar(255) not null,
    member_id uuid not null,
    access_token varchar(255) not null,
    primary key (id),
    constraint FK_oauth2_member_table_member foreign key(member_id) references member(id) on delete cascade
);