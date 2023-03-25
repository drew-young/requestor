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
    alive BOOLEAN DEFAULT false,
    lastCheckIn DATETIME DEFAULT NULL,
    FOREIGN KEY (hostname) REFERENCES hostnames(hostname)
);

CREATE TABLE commands (
    id INT AUTO_INCREMENT PRIMARY KEY,
    host_id INT NOT NULL,
    command VARCHAR(255) NOT NULL,
    response VARCHAR(255) DEFAULT '',
    acknowledged BOOLEAN DEFAULT false,
    FOREIGN KEY (host_id) REFERENCES hosts(id)
);

INSERT INTO hostnames (hostname, ipFormat) VALUES ('localhost', '127.0.0.X');

INSERT INTO hosts (identifier, hostname) VALUES ('localhost.1', 'localhost');

SELECT id FROM hosts WHERE identifier = 'localhost.1';

INSERT INTO commands (host_id, command)
VALUES (
    (SELECT id FROM hosts WHERE identifier = 'localhost.1'),
    'echo "Hello World"'
);