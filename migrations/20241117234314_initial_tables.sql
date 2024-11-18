-- Add migration script heremigrat
CREATE TABLE IF NOT EXISTS hourly_samples (
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    avg_bytes_per_second_in REAL NOT NULL,
    avg_bytes_per_second_out REAL NOT NULL,
    total_bytes_received INTEGER NOT NULL,
    total_bytes_sent INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS daily_samples (
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    avg_bytes_per_second_in REAL NOT NULL,
    avg_bytes_per_second_out REAL NOT NULL,
    total_bytes_received INTEGER NOT NULL,
    total_bytes_sent INTEGER NOT NULL
);