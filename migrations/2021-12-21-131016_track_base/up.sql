-- Your SQL goes here
CREATE TABLE track_base (
    id SERIAL PRIMARY KEY
   ,category smallint
   ,description TEXT
   ,start_time timestamp NOT NULL
   ,end_time timestamp NOT NULL
   ,timespan timestamp NOT NULL
   ,create_user varchar(50) NOT NULL
   ,created timestamp NOT NULL
   ,updated timestamp NOT NULL
)