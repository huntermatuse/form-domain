-- As a style choice, we prefer to avoid plurals in table names, mainly because it makes queries read better.
-- Tables get an integer auto-incrementing `id` and a `{table}_id` uuid column used throughout
-- the application. The table `id` column is database-only, primarily for indexes. Foreign keys
-- and joins use the uuid column for application usage.

-- Log Capture Table used to store any logs we may need
create schema log;

create table log.events (
    "timestamp" integer not null,
    loglevel text not null,
    source text not null,
    category text,
    message text not null
);

create index events_category_idx on log.events (category);

create index events_loglevel_idx on log.events (loglevel);

create index events_source_idx on log.events (source);

create index events_timestamp_idx on log.events ("timestamp");
