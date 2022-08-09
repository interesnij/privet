-- Your SQL goes here
-- help -------
---------------
---------------
CREATE TABLE help_item_categories (
    id    SERIAL PRIMARY KEY,
    title VARCHAR(200) NOT NULL
);

CREATE TABLE help_items (
    id          SERIAL PRIMARY KEY,
    category_id INT NOT NULL,
    title       VARCHAR(200) NOT NULL,
    content     VARCHAR(1000) NOT NULL
);
