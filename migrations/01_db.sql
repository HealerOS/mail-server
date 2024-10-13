create table subscriptions
(
    id            uuid         not null,
    primary key (id),
    email         varchar(100) not null unique,
    username      varchar(20)  not null,
    subscribed_at timestamptz  not null

);