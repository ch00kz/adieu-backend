create table "games" (
    "id" uuid primary key default gen_random_uuid (),
    "word" text not null,
    "created_at" timestamptz not null default now ()
);

create table "players" (
    "id" uuid primary key default gen_random_uuid (),
    "username" text not null,
    "game_id" uuid not null references games (id) on delete cascade,
    "created_at" timestamptz not null default now ()
);

create unique index on players (game_id, username);

create index on players (game_id);

create table "guesses" (
    "id" uuid primary key default gen_random_uuid (),
    "player_id" uuid not null references players (id) on delete cascade,
    "guess" text not null,
    "is_winning_guess" boolean not null,
    "created_at" timestamptz not null default now ()
);

create index on guesses (player_id);
