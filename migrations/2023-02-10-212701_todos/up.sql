CREATE TABLE todos
(
    id          INTEGER PRIMARY KEY NOT NULL,
    description TEXT                NOT NULL,
    completed   BOOLEAN             NOT NULL DEFAULT FALSE,
    session_id  INTEGER             NOT NULL
);

