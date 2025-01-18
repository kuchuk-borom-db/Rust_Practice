CREATE TABLE IF NOT EXISTS logs (
id VARCHAR(100) PRIMARY KEY,
operation_id VARCHAR(100) NOT NULL,
block_name VARCHAR(100) NOT NULL,
log_type VARCHAR(50) NOT NULL,
log_value VARCHAR(250),
sequence INT NOT NULL
);

CREATE TABLE IF NOT EXISTS operations (
    id VARCHAR(100) PRIMARY KEY,
    created TIMESTAMP NOT NULL,
    updated TIMESTAMP NOT NULL
);
