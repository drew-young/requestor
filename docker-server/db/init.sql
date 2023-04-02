USE requestor_db;



CREATE TABLE IF NOT EXISTS hosts (
    id INT AUTO_INCREMENT PRIMARY KEY,
    identifier VARCHAR(255) NOT NULL,
    hostname VARCHAR(255) NOT NULL,
    ip VARCHAR(255) NOT NULL, 
    alive BOOLEAN DEFAULT false,
    lastCheckIn DATETIME DEFAULT NULL
);

CREATE TABLE IF NOT EXISTS commands (
    id INT AUTO_INCREMENT PRIMARY KEY,
    host_id INT NOT NULL,
    command VARCHAR(255) NOT NULL,
    response longtext DEFAULT NULL,
    acknowledged BOOLEAN DEFAULT false
);

CREATE TABLE IF NOT EXISTS teams (
  team_number INT NOT NULL,
  ip_addresses TEXT NOT NULL,
  PRIMARY KEY (team_number)
);

CREATE TABLE IF NOT EXISTS hostnames (
    hostname VARCHAR(255) NOT NULL,
    ip_addresses TEXT NOT NULL,
    PRIMARY KEY (hostname)
);

delimiter //

CREATE FUNCTION IF NOT EXISTS issueCommand(host_identifier VARCHAR(255), command VARCHAR(255)) RETURNS INT
DETERMINISTIC
BEGIN
    DECLARE host_id INT;
    DECLARE command_id INT;
    SELECT id INTO host_id FROM hosts WHERE identifier = host_identifier;
    INSERT INTO commands (host_id, command) VALUES (host_id, command);
    SELECT LAST_INSERT_ID() INTO command_id;
    RETURN command_id;
END//

CREATE FUNCTION IF NOT EXISTS updateCommandResponse(command_id INT, response longtext) RETURNS INT
DETERMINISTIC
BEGIN
    UPDATE commands SET response = response WHERE id = command_id;
    RETURN command_id;
END//

CREATE FUNCTION IF NOT EXISTS checkIn(host_identifier VARCHAR(255)) RETURNS VARCHAR(255)
DETERMINISTIC
BEGIN
    DECLARE host_ip VARCHAR(255);
    DECLARE host_id INT;
    SELECT ip INTO host_ip FROM hosts WHERE identifier = host_identifier;
    SELECT id INTO host_id FROM hosts WHERE identifier = host_identifier;
    UPDATE hosts SET alive = true, lastCheckIn = NOW() WHERE id = host_id;
    RETURN host_ip;
END//

CREATE FUNCTION IF NOT EXISTS newHost(host_ip VARCHAR(15)) RETURNS VARCHAR(255)
DETERMINISTIC
BEGIN
    DECLARE team_number INT;
    DECLARE host_identifier VARCHAR(100);
    DECLARE host_hostname VARCHAR(255);
    SELECT check_ip_in_team(host_ip) INTO team_number;
    IF team_number IS NULL THEN
      SET team_number = 0;
    END IF;
    SELECT check_ip_in_hostname(host_ip) INTO host_hostname;
    IF host_hostname IS NULL THEN
      SET host_hostname = host_ip;
    END IF;
    SET host_identifier = CONCAT(SUBSTRING(host_hostname, 1, 200), '.', CAST(team_number AS CHAR));
    IF NOT EXISTS(SELECT * FROM hosts WHERE identifier = host_identifier AND hostname = host_hostname AND ip = host_ip) THEN
      INSERT INTO hosts (identifier, hostname, ip) VALUES (host_identifier, host_hostname, host_ip);
    END IF;
    RETURN host_identifier;
END//

CREATE FUNCTION IF NOT EXISTS check_ip_in_team(ip_address VARCHAR(15))
RETURNS INT
DETERMINISTIC
BEGIN
  DECLARE team_count INT;
  DECLARE i INT DEFAULT 1;
  DECLARE team_ips TEXT;
  DECLARE found BOOLEAN DEFAULT FALSE;
  DECLARE team_number INT DEFAULT NULL;

  SELECT COUNT(*) INTO team_count FROM teams;

  WHILE i <= team_count AND NOT found DO
    SELECT ip_addresses INTO team_ips FROM teams WHERE team_number = i;
    IF FIND_IN_SET(ip_address, team_ips) > 0 THEN
      SET found = TRUE;
      SET team_number = i;
    END IF;
    SET i = i + 1;
  END WHILE;

  RETURN team_number;
END //

CREATE FUNCTION IF NOT EXISTS check_ip_in_hostname(ip VARCHAR(255))
RETURNS VARCHAR(255)
DETERMINISTIC
BEGIN
  DECLARE hostname_count INT;
  DECLARE i INT DEFAULT 1;
  DECLARE hostname_ips TEXT;
  DECLARE found BOOLEAN DEFAULT FALSE;
  DECLARE hostname VARCHAR(255) DEFAULT NULL;

  SELECT COUNT(*) INTO hostname_count FROM hostnames;

  WHILE i <= hostname_count AND NOT found DO
    SELECT ip_addresses INTO hostname_ips FROM hostnames WHERE hostname = i;
    IF FIND_IN_SET(ip_address, hostname_ips) > 0 THEN
      SET found = TRUE;
      SET hostname = i;
    END IF;
    SET i = i + 1;
  END WHILE;

  RETURN hostname;
END //

CREATE FUNCTION IF NOT EXISTS getQueuedCommands(host_identifier VARCHAR(255)) RETURNS TEXT
DETERMINISTIC
BEGIN
  DECLARE host_id INT;
  SELECT id INTO host_id FROM hosts WHERE identifier = host_identifier;
RETURN IFNULL((SELECT GROUP_CONCAT(id SEPARATOR ';') FROM commands WHERE host_id = host_id AND acknowledged = false), 'NONE');
END//

CREATE FUNCTION IF NOT EXISTS getCommand(cmd_id INT) RETURNS TEXT
DETERMINISTIC
BEGIN
  UPDATE commands SET acknowledged = true WHERE id = cmd_id;
  RETURN IFNULL((SELECT command FROM commands WHERE id = cmd_id), 'NONE');
END//

CREATE TRIGGER update_alive
BEFORE UPDATE ON hosts
FOR EACH ROW
BEGIN
    IF (NEW.lastCheckIn < DATE_SUB(NOW(), INTERVAL 15 SECOND)) THEN
        SET NEW.alive = 0;
    END IF;
END//

delimiter ;