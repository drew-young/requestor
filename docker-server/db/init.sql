USE requestor_db;

CREATE TABLE hostnames (
    id INT AUTO_INCREMENT PRIMARY KEY,
    hostname VARCHAR(255) NOT NULL,
    ipFormat VARCHAR(255) NOT NULL
);

CREATE TABLE hosts (
    id INT AUTO_INCREMENT PRIMARY KEY,
    identifier VARCHAR(255) NOT NULL,
    hostname VARCHAR(255) NOT NULL,
    hostname_id INT NOT NULL,
    alive BOOLEAN NOT NULL,
    lastCheckIn DATETIME NOT NULL,
    FOREIGN KEY (hostname_id) REFERENCES hostname(id)
);

CREATE TABLE commands (
    id INT AUTO_INCREMENT PRIMARY KEY,
    host_id INT NOT NULL,
    command VARCHAR(255) NOT NULL,
    response VARCHAR(255) NOT NULL,
    acknowledged BOOLEAN NOT NULL,
    FOREIGN KEY (host_id) REFERENCES host(id)
);

CREATE FUNCTION update_command_response(
    command_id INTEGER,
    response_text TEXT
)
RETURNS VOID AS $$
BEGIN
    UPDATE commands
    SET response = response_text
    WHERE id = command_id;
END;
$$ LANGUAGE plpgsql;
