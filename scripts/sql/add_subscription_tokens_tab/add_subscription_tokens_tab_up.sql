create table if not exists subscription_tokens
(
    id                 uuid primary key not null,
    subscription_token varchar(200)     not null,
    subscriber_id      uuid             not null references subscriptions (id)
);
