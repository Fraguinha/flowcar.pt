-- Enable uuid extension
  CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

  -- Create custom types
  CREATE TYPE vehicle_category AS ENUM ('suv', 'van', 'sedan', 'coupe', 'cabrio', 'motorcycle');
  CREATE TYPE vehicle_fuel AS ENUM ('electric', 'hybrid', 'diesel', 'gas');
  CREATE TYPE vehicle_transmission AS ENUM ('manual', 'automatic');

  -- Create models table
  CREATE TABLE models (
    id UUID PRIMARY KEY,
    make VARCHAR(255) NOT NULL,
    model VARCHAR(255) NOT NULL
  );

  -- Create vehicles table
  CREATE TABLE vehicles (
    id UUID PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    image JSONB NOT NULL,
    video JSONB NOT NULL,
    category vehicle_category NOT NULL,
    fuel vehicle_fuel NOT NULL,
    transmission vehicle_transmission NOT NULL,
    price INT NOT NULL,
    price_monthly INT NOT NULL,
    year INT NOT NULL,
    mileage INT NOT NULL,
    horsepower INT NOT NULL,
    displacement INT NOT NULL,
    extra TEXT[] NOT NULL,
    model UUID REFERENCES models(id)
  );

-- Create users table
CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL
);

-- Create permission_tokens table
CREATE TABLE permission_tokens (
    token VARCHAR(255) PRIMARY KEY,
    user_id BIGINT REFERENCES users(id) ON DELETE CASCADE
);

-- Create admin
INSERT INTO users (id, username, password) VALUES (1, 'admin', '$2b$12$gsPfnG/Euf2u6ncTLnVZy.Z5fWXOLFcXeMGHLd1sjPitYD2S1tul2');
ALTER SEQUENCE users_id_seq RESTART WITH 2;

-- Example models
INSERT INTO models (id, make, model) VALUES (UUID_GENERATE_V4(), 'Mini', 'Cooper Cabrio');
INSERT INTO models (id, make, model) VALUES (UUID_GENERATE_V4(), 'BMW', 'M4 Coupe DKG');

-- Example vehicles
INSERT INTO vehicles (id, title, image, video, category, fuel, transmission, price, price_monthly, year, mileage, horsepower, displacement, extra, model)
VALUES (
  UUID_GENERATE_V4(),
  'Mini Cooper Cabrio',
  '[
    { "src": "/images/Mini-Cooper-1.jpeg", "alt": "Mini Cooper Cabrio" },
    { "src": "/images/Mini-Cooper-2.jpeg", "alt": "Mini Cooper Cabrio" },
    { "src": "/images/Mini-Cooper-3.jpeg", "alt": "Mini Cooper Cabrio" },
    { "src": "/images/Mini-Cooper-4.jpeg", "alt": "Mini Cooper Cabrio" },
    { "src": "/images/Mini-Cooper-5.jpeg", "alt": "Mini Cooper Cabrio" },
    { "src": "/images/Mini-Cooper-6.jpeg", "alt": "Mini Cooper Cabrio" },
    { "src": "/images/Mini-Cooper-7.jpeg", "alt": "Mini Cooper Cabrio" },
    { "src": "/images/Mini-Cooper-8.jpeg", "alt": "Mini Cooper Cabrio" },
    { "src": "/images/Mini-Cooper-9.jpeg", "alt": "Mini Cooper Cabrio" }
  ]',
  '{ "src": "/videos/Mini-Cooper.mp4", "alt": "Mini Cooper Cabrio" }',
  'cabrio',
  'gas',
  'manual',
  9950,
  150,
  2010,
  166000,
  98,
  1598,
  '{"Bancos aquecidos", "AUX", "A/C individual", "Sensores de estacionamento traseiros"}',
  (SELECT id FROM models WHERE make='Mini' AND model='Cooper Cabrio')
);

INSERT INTO vehicles (id, title, image, video, category, fuel, transmission, price, price_monthly, year, mileage, horsepower, displacement, extra, model)
VALUES (
  UUID_GENERATE_V4(),
  'BMW M4 Coupe DKG',
  '[
    { "src": "/images/BMW-M4-Coupe-DKG-1.jpeg", "alt": "BMW M4 Coupe" },
    { "src": "/images/BMW-M4-Coupe-DKG-2.jpeg", "alt": "BMW M4 Coupe" },
    { "src": "/images/BMW-M4-Coupe-DKG-3.jpeg", "alt": "BMW M4 Coupe" },
    { "src": "/images/BMW-M4-Coupe-DKG-4.jpeg", "alt": "BMW M4 Coupe" },
    { "src": "/images/BMW-M4-Coupe-DKG-5.jpeg", "alt": "BMW M4 Coupe" },
    { "src": "/images/BMW-M4-Coupe-DKG-6.jpeg", "alt": "BMW M4 Coupe" },
    { "src": "/images/BMW-M4-Coupe-DKG-7.jpeg", "alt": "BMW M4 Coupe" },
    { "src": "/images/BMW-M4-Coupe-DKG-8.jpeg", "alt": "BMW M4 Coupe" },
    { "src": "/images/BMW-M4-Coupe-DKG-9.jpeg", "alt": "BMW M4 Coupe" },
    { "src": "/images/BMW-M4-Coupe-DKG-10.jpeg", "alt": "BMW M4 Coupe" }
  ]',
  '{}',
  'coupe',
  'gas',
  'manual',
  70000,
  300,
  2015,
  87000,
  430,
  2979,
  '{"Sistema de navegação", "Sistema de Bluetooth e kit mãos livres", "Bancos em pele aquecido M e elétricos", "sensores de estacionamento traseiros e frontais", "Mala e tejadilho em Carbono"}',
  (SELECT id FROM models WHERE make='BMW' AND model='M4 Coupe DKG')
);
