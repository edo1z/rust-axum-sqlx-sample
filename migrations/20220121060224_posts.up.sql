CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users (id),
    category_id INTEGER REFERENCES categories (id),
    title VARCHAR(140) NOT NULL,
    content TEXT 
)
