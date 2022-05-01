SELECT 'CREATE DATABASE pharmacy' WHERE NOT EXISTS (SELECT FROM pg_database WHERE datname = 'pharmacy');

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
-- CREATE USER IF NOT EXISTS 'fume' @'%' IDENTIFIED BY '1324';

CREATE TABLE IF NOT EXISTS t_user (
    id BIGSERIAL PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    created_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    password TEXT NOT NULL UNIQUE
);

INSERT INTO t_user(id, username, email, created_date, password)
VALUES (1, 'fume', 'test@test.ru', current_timestamp, '$2y$08$a2eRpoUeJxoDc2lPyEuEEu3wvvIgi6CAV33b/Eua2yCyGoAFnhLia')
ON CONFLICT DO NOTHING;