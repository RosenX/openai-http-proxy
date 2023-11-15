-- Add up migration script here

CREATE TABLE purchase_detail (
    user_id VARCHAR(100) NOT NULL,
    product_id VARCHAR(100) NOT NULL,
    purchase_time TIMESTAMP(0) WITH TIME ZONE NOT NULL,
    is_cancelled BOOLEAN NOT NULL DEFAULT FALSE,
    source VARCHAR(100) NOT NULL,
    PRIMARY KEY (user_id, purchase_time, product_id)
);