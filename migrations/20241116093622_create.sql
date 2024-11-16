-- Add migration script here

CREATE TABLE stats (
    minecraft_id UUID NOT NULL,
    server_name VARCHAR(100) NOT NULL,
    playtime INT,
    blocks_broken INT,
    blocks_placed INT,
    deaths INT,

    PRIMARY KEY (minecraft_id, server_name)
);