-- Your SQL goes here
CREATE TABLE videos (
    id serial not null,
    title character varying(255) NOT NULL,
    description text NOT NULL,
    removed boolean NOT NULL,
    CONSTRAINT videos_pkey PRIMARY KEY (id)
)