CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

/*
    Tables
*/

create table users
(
    id          uuid      default uuid_generate_v4() not null
        constraint users_pk
            primary key,
    nick        text                                 not null,
    gender      text                                 not null,
    hash        bytea                                not null,
    salt        varchar(128)                         not null,
    email       text                                 not null,
    created_at  timestamp default CURRENT_TIMESTAMP  not null,
    description text                                 not null
);

create unique index users_email_uindex
    on users (email);

create table game_requests
(
    id           uuid      default uuid_generate_v4() not null
        constraint game_requests_pk
            primary key,
    name         text                                 not null,
    last_played  uuid                                 not null
        constraint game_requests_users_id_fk
            references users,
    created_at   timestamp default CURRENT_TIMESTAMP  not null,
    moves_needed smallint                             not null
);

create table users_to_game_requests
(
    user_id         uuid                  not null
        constraint users_to_game_requests_users_id_fk
            references users,
    game_request_id uuid                  not null
        constraint users_to_game_requests_game_requests_id_fk
            references game_requests,
    accepted        boolean default false not null,
    constraint users_to_game_requests_pkey
        primary key (user_id, game_request_id)
);

create table roles
(
    id   smallserial not null
        constraint roles_pk
            primary key,
    name text        not null
);

create unique index roles_name_uindex
    on roles (name);

create table roles_to_users
(
    user_id uuid    not null
        constraint roles_to_users_users_id_fk
            references users,
    role_id integer not null
        constraint roles_to_users_roles_id_fk
            references roles,
    constraint roles_to_users_pkey
        primary key (user_id, role_id)
);

create table games
(
    id           uuid      default uuid_generate_v4() not null
        constraint games_pk
            primary key,
    name         text                                 not null,
    ended        boolean   default false              not null,
    last_played  uuid                                 not null
        constraint games_users_id_fk_2
            references users,
    data         bytea                                not null,
    created_at   timestamp default CURRENT_TIMESTAMP  not null,
    moves_needed smallint                             not null,
    winner       uuid
);

create table games_to_users
(
    game_id uuid not null
        constraint games_to_users_games_id_fk
            references games,
    user_id uuid not null
        constraint games_to_users_users_id_fk
            references users,
    constraint games_to_users_pkey
        primary key (user_id, game_id)
);

/*
    Procedures
*/

create procedure update_invite(_user_id uuid, _game_request_id uuid, _accepted boolean, _data bytea)
    language plpgsql
as
$$
declare
    v_ready   bool;
    v_game_id uuid;
    v_exists  bool;
begin

    select exists(select * from users_to_game_requests where game_request_id = _game_request_id and user_id = _user_id)
    into v_exists;

    if not v_exists then
        raise exception 'User with id ''%'' is not part of game request with id ''%'' or game request with id ''%'' doesn''t exists', _user_id, _game_request_id, _game_request_id;
    end if;

    if _accepted then
        update users_to_game_requests
        set accepted = true
        where user_id = _user_id
          and game_request_id = _game_request_id;


        select not exists(select *
                          from users_to_game_requests
                          where game_request_id = _game_request_id
                            and not accepted)
        into v_ready;

        if v_ready then
            insert into games (name, data, last_played, moves_needed)
            select name, _data, last_played, moves_needed
            from game_requests
            where game_requests.id = _game_request_id
            returning games.id
                into v_game_id;


            insert into games_to_users (user_id, game_id)
            select users_to_game_requests.user_id, v_game_id
            from users_to_game_requests
            where game_request_id = _game_request_id;

            delete from users_to_game_requests where game_request_id = _game_request_id;
            delete from game_requests where id = _game_request_id;
        end if;
    else
        delete from users_to_game_requests where game_request_id = _game_request_id;
        delete from game_requests where id = _game_request_id;
    end if;

    commit;
end;
$$;

create procedure delete_old_game_requests()
    language plpgsql
as
$$
begin
    delete from game_requests where created_at + interval '1 day' > current_timestamp;
end;
$$;

create procedure new_game_request(_name text, _last_played uuid, _users_id uuid[], _moves_needed smallint)
    language plpgsql
as
$$
declare
    v_game_request_id uuid;
begin
    insert into game_requests (name, last_played, moves_needed)
    values (_name, _last_played, _moves_needed)
    returning game_requests.id into v_game_request_id;

    insert into users_to_game_requests (user_id, game_request_id)
    select user_id__, v_game_request_id
    from unnest(_users_id) user_id__;
    commit;
end;
$$;

create procedure update_user(_id uuid, _nick text DEFAULT NULL::text, _gender text DEFAULT NULL::text,
                             _email text DEFAULT NULL::text, _hash bytea DEFAULT NULL::bytea,
                             _salt character varying DEFAULT NULL::character varying,
                             _roles integer[] DEFAULT NULL::integer[], _description text DEFAULT NULL::text)
    language plpgsql
as
$$
begin
    if _nick is not null then
        update users
        set nick = $2
        where id = $1;
    end if;
    if _gender is not null then
        update users
        set gender = $3
        where id = $1;
    end if;
    if _email is not null then
        update users
        set email = $4
        where id = $1;
    end if;
    if _hash is not null then
        update users
        set hash = $5
        where id = $1;
    end if;
    if _salt is not null then
        update users
        set salt = $6
        where id = $1;
    end if;
    if _description is not null then
        update users
        set description = _description
        where id = _id;
    end if;

    if _roles is not null then
        delete from roles_to_users where user_id = _id;
        insert into roles_to_users (user_id, role_id) select _id, role_id__ FROM unnest(_roles) role_id__;
    end if;

    commit;
end;
$$;

/*
    Data
*/

INSERT INTO users (nick, gender, hash, salt, email, description)
VALUES ('root', 'root', E'\\xDF2922D9C170DD88507689A2E32187AA49AE508A9AFCDF6929CC81657F1AF7ED',
        'z6^VNSo8lJh1$2NKxaJ&(kkBeEPH5JH!c0g%OKv*~&r9KPX*EX*^E!2bmLThkqGj4rT1!hkZ4zmlTpN1FkEoAj38)gMipd5l7TJBOgrRlyTvPi^9wDf)5$c$3CFF78^d',
        'r@r.r', 'root');

INSERT INTO roles (name)
VALUES ('Admin');
INSERT INTO roles (name)
VALUES ('Banned');

INSERT INTO roles_to_users (user_id, role_id) VALUES ((select id from users where email = 'r@r.r'), (select id from roles where name = 'Admin'));
