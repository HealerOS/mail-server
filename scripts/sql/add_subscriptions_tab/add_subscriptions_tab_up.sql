create type public.status_enum as enum ('Waiting', 'Confirmed');
alter type public.status_enum owner to postgres;

create table subscriptions
(
    id            uuid                                       not null,
    primary key (id),
    email         varchar(100)                               not null unique,
    username      varchar(20)                                not null,
    subscribed_at timestamptz                                not null,
    status        status_enum default 'Waiting'::status_enum not null
);
