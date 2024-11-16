-- Add migration script here

CREATE TABLE stats (
    minecraft_uuid UUID NOT NULL,
    server VARCHAR(100) NOT NULL,
    playtime INT,
    blocks_broken INT,
    blocks_placed INT,
    deaths INT,
    last_updated TIMESTAMP NOT NULL,

    PRIMARY KEY (minecraft_uuid, server)
);