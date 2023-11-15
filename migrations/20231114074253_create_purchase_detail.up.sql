-- Add up migration script here

CREATE TABLE purchase_detail (
    user_id VARCHAR(100) NOT NULL,
    product_id VARCHAR(100) NOT NULL,
    transaction_date TIMESTAMP(0) WITH TIME ZONE NOT NULL,
    verified_data VARCHAR(2000),
    is_cancelled BOOLEAN NOT NULL DEFAULT FALSE,
    source VARCHAR(100) NOT NULL,
    PRIMARY KEY (user_id, transaction_date, product_id)
);