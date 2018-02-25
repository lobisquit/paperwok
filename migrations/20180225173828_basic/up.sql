CREATE TABLE tags (
name string
);

CREATE TABLE folders (
id integer PRIMARY KEY AUTOINCREMENT,
year integer,
title integer,
binder string
);

CREATE TABLE tags_folders (
tag_name string,
folder_id integer
);

CREATE TABLE documents (
id integer,
title string,
folder_id string
);
