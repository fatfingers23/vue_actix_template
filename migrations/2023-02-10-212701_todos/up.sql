CREATE TABLE todos (
                       id INTEGER PRIMARY KEY NOT NULL,
                       title VARCHAR NOT NULL,
                       description TEXT NOT NULL,
                       completed BOOLEAN NOT NULL DEFAULT FALSE
);

