USE requestor_db;

CREATE TABLE hosts (
    id INT AUTO_INCREMENT PRIMARY KEY,
    identifier VARCHAR(255) NOT NULL,
    hostname VARCHAR(255) NOT NULL,
    ip VARCHAR(255) NOT NULL, 
    alive BOOLEAN DEFAULT false,
    lastCheckIn DATETIME DEFAULT NULL
);

CREATE TABLE commands (
    id INT AUTO_INCREMENT PRIMARY KEY,
    host_id INT NOT NULL,
    command VARCHAR(255) NOT NULL,
    response VARCHAR(255) DEFAULT '',
    acknowledged BOOLEAN DEFAULT false
);

delimiter //

CREATE FUNCTION issueCommand(host_identifier VARCHAR(255), command VARCHAR(255)) RETURNS INT
DETERMINISTIC
BEGIN
    DECLARE host_id INT;
    DECLARE command_id INT;
    SELECT id INTO host_id FROM hosts WHERE identifier = host_identifier;
    INSERT INTO commands (host_id, command) VALUES (host_id, command);
    SELECT LAST_INSERT_ID() INTO command_id;
    RETURN command_id;
END//

CREATE FUNCTION updateCommandResponse(command_id INT, response VARCHAR(255)) RETURNS INT
DETERMINISTIC
BEGIN
    UPDATE commands SET response = response WHERE id = command_id;
    RETURN command_id;
END//

CREATE FUNCTION checkIn(host_identifier VARCHAR(255)) RETURNS INT
DETERMINISTIC
BEGIN
    DECLARE host_id INT;
    SELECT id INTO host_id FROM hosts WHERE identifier = host_identifier;
    UPDATE hosts SET alive = true, lastCheckIn = NOW() WHERE id = host_id;
    RETURN host_id;
END//

delimiter ;