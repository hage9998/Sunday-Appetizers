CREATE SCHEMA IF NOT EXISTS pate;

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS pate.customers(
  customer_id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
  first_name TEXT NOT NULL,
  last_name TEXT NOT NULL,
  customer_phone TEXT NOT NULL,
  customer_email TEXT NOT NULL
);

CREATE TYPE pate.types_prod AS ENUM ('BEBIDA', 'SALGADOS', 'DOCE');

CREATE TABLE IF NOT EXISTS pate.products(
  product_id SERIAL PRIMARY KEY,
  product_price NUMERIC(5,2),
  product_name TEXT,
  product_type pate.types_prod
);

CREATE TYPE pate.order_status AS ENUM ('CONCLUÍDO', 'EM PREPARO', 'Á CAMINHO', 'PEDIDO ACEITO', 'PEDIDO RECUSADO');

CREATE TABLE IF NOT EXISTS pate.customer_orders_status(
  status_id serial PRIMARY KEY,
  status_name pate.order_status NOT NULL,
  status_description TEXT NULL
);

CREATE TABLE IF NOT EXISTS pate.customer_orders(
  order_id SERIAL PRIMARY KEY,
  customer_id uuid REFERENCES pate.customers,
  order_status_code INT REFERENCES pate.customer_orders_status,
  date_order_placed TIMESTAMP,
  date_order_paid TIMESTAMP
);

CREATE TABLE IF NOT EXISTS pate.customer_orders_product(
  order_id INT REFERENCES pate.customer_orders,
  product_id INT REFERENCES pate.products,
  quantity INT NOT NULL,
  comments TEXT,
  PRIMARY KEY(order_id, product_id)
);

CREATE TABLE IF NOT EXISTS pate.adresses(
  address_id SERIAL PRIMARY KEY,
  address_street TEXT NOT NULL,
  address_number INT NOT NULL,
  address_district TEXT NOT NULL,
  zip_postcode TEXT NOT NULL,
  city_name TEXT NOT NULL,
  state_name TEXT NOT NULL,
  country_name TEXT NOT NULL,
  other_address_details TEXT
);

CREATE TABLE IF NOT EXISTS pate.customer_adresses(
	customer_id uuid REFERENCES pate.customers,
	address_id INT REFERENCES pate.adresses,
	custormer_address_id SERIAL,
	PRIMARY KEY(customer_id, address_id, custormer_address_id)
);