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
);

CREATE TABLE commands (
    id INT AUTO_INCREMENT PRIMARY KEY,
    host_id INT NOT NULL,
    command VARCHAR(255) NOT NULL,
    response VARCHAR(255) DEFAULT '',
    acknowledged BOOLEAN DEFAULT false,
);

INSERT INTO hostnames (hostname, ipFormat) VALUES ('localhost', '127.0.0.X');

INSERT INTO hosts (identifier, hostname) VALUES ('localhost.1', 'localhost');

SELECT id FROM hosts WHERE identifier = 'localhost.1';

INSERT INTO commands (host_id, command)
VALUES (
    (SELECT id FROM hosts WHERE identifier = 'localhost.1'),
    'echo "Hello World"'
);

ERROR 1064 (42000): You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'ls /' at line 1
mysql> source /docker-entrypoint-initdb.d/init.sql;
Database changed
ERROR 1050 (42S01): Table 'hostnames' already exists
ERROR 1822 (HY000): Failed to add the foreign key constraint. Missing index for constraint 'hosts_ibfk_1' in the referenced table 'hostnames'
ERROR 1824 (HY000): Failed to open the referenced table 'hosts'
Query OK, 1 row affected (0.02 sec)

ERROR 1146 (42S02): Table 'requestor_db.hosts' doesn't exist
ERROR 1146 (42S02): Table 'requestor_db.hosts' doesn't exist
ERROR 1146 (42S02): Table 'requestor_db.commands' doesn't exist