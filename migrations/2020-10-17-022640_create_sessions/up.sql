CREATE TABLE sessions
(
    id            SERIAL PRIMARY KEY,
    user_id       INT       NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    refresh_token CHAR(8)   NOT NULL,
    fingerprint   TEXT      NOT NULL,
    expires       TIMESTAMP NOT NULL
)
