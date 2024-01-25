CREATE TABLE IF NOT EXISTS products(
    product_id serial PRIMARY KEY,
    product_name varchar(64) NOT NULL,
    description text,
    price DOUBLE PRECISION NOT NULL,
    stock_quantity integer NOT NULL,
    category_id integer
    -- created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    -- updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
);

