SELECT 'CREATE DATABASE pharmacy' WHERE NOT EXISTS (SELECT FROM pg_database WHERE datname = 'pharmacy');

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Only revoke if user 'my_user exists'
-- DO $$DECLARE count int;
-- BEGIN
-- SELECT count(*) INTO count FROM pg_roles WHERE rolname = 'fume';
-- IF count > 0 THEN
--     EXECUTE 'REVOKE CONNECT ON DATABASE "pharmacy" FROM fume';
-- END IF;
-- END$$;

-- -- No privileges left, now it should be possible to drop
-- DROP ROLE IF EXISTS fume;

-- CREATE ROLE fume LOGIN PASSWORD '1324';
-- GRANT ALL PRIVILEGES ON DATABASE "pharmacy" to fume;

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

CREATE TABLE IF NOT EXISTS creator(
    id BIGSERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    license_num INTEGER
);

CREATE TABLE IF NOT EXISTS medicine (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    cost REAL DEFAULT 0,
    description TEXT,
    created_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    fk_creator_id BIGSERIAL,
    FOREIGN KEY (fk_creator_id) REFERENCES creator (id)
);

CREATE TABLE IF NOT EXISTS pharmacy (
    id BIGSERIAL PRIMARY KEY,
    name TEXT,
    location TEXT
);

CREATE TABLE IF NOT EXISTS medicine_pharmacy (
    fk_medicine_id TEXT NOT NULL,
    fk_pharmacy_id BIGSERIAL,
    FOREIGN KEY (fk_medicine_id) REFERENCES medicine (id),
    FOREIGN KEY (fk_pharmacy_id) REFERENCES pharmacy (id)
);
