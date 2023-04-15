PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS roles(
    id BIGINT NOT NULL UNIQUE,
    guild_id BIGINT NOT NULL,
    color CHAR(6) NOT NULL UNIQUE,
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS members(
    id BIGINT NOT NULL,
    guild_id BIGINT NOT NULL,
    role_id BIGINT NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (role_id) REFERENCES roles(id),
    UNIQUE (id, guild_id, role_id)
);
