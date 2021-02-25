create table if not exists author
(
    id               integer auto_increment,
    name             varchar(255) not null,
    username         varchar(255) not null,
    resume           text         not null,
    company          varchar(255) null,
    years_experience int          null,
    country          varchar(255) null,
    constraint author_pk
        primary key (id)
);


create table if not exists article
(
    id        bigint auto_increment
        primary key,
    title     varchar(255) not null,
    body      text         not null,
    author    bigint       not null,
    create_at timestamp    not null default now(),
    update_at timestamp    not null default now(),
    constraint article_author_id_fk
        foreign key (author) references author (id)
);

