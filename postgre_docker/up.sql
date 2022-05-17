SELECT 'CREATE DATABASE pharmacy' WHERE NOT EXISTS (SELECT FROM pg_database WHERE datname = 'pharmacy');

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

REVOKE pg_read_all_data FROM test;
REVOKE pg_write_all_data FROM test;
REVOKE ALL PRIVILEGES ON DATABASE "pharmacy" FROM test;
DROP ROLE IF EXISTS test;
CREATE ROLE test LOGIN PASSWORD '1324';
GRANT pg_read_all_data TO test;
GRANT pg_write_all_data TO test;

CREATE TABLE IF NOT EXISTS t_user (
    id BIGSERIAL PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    created_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    password TEXT NOT NULL UNIQUE
);

INSERT INTO t_user(id, username, email, created_date, password)
VALUES (1, 'test', 'test@test.ru', current_timestamp, '$2y$08$a2eRpoUeJxoDc2lPyEuEEu3wvvIgi6CAV33b/Eua2yCyGoAFnhLia')
ON CONFLICT DO NOTHING;

CREATE TABLE IF NOT EXISTS medicine (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    creator_name TEXT NOT NULL,
    cost REAL DEFAULT 0,
    description TEXT,
    created_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);