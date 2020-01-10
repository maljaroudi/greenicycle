-- Your SQL goes here
CREATE TABLE task (
    id INTEGER NOT NULL,
    _name TEXT NOT NULL,
    category TEXT CHECK( category IN ('Food', 'Drink', 'Metal', 'Paint', 'Paper', 'Automotive', 'Electronics', 'Restaurant', 'Other' )) NOT NULL,
    recyclable TEXT CHECK ( recyclable IN ('0','1', 'True', 'False')) NOT NULL,
    PRIMARY KEY (id)
);
