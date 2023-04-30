CREATE TABLE packages (
  package_id SERIAL,
  url VARCHAR,
  version VARCHAR NOT NULL,
  package_name VARCHAR NOT NULL,
  jsprogram VARCHAR,
  content VARCHAR,
  metric_one REAL,
  metric_two REAL,
  metric_three REAL,
  metric_four REAL,
  metric_five REAL,
  metric_six REAL,
  metric_seven REAL,
  total_score REAL,
  PRIMARY KEY(package_id)
);

CREATE TABLE users (
  user_id SERIAL,
  user_name VARCHAR NOT NULL,
  isAdmin BOOLEAN NOT NULL,
  PRIMARY KEY(user_id)
);

CREATE TABLE requests (
  request_id SERIAL,
  request_type VARCHAR NOT NULL,
  PRIMARY KEY(request_id)
);

CREATE TABLE groups (
  group_id SERIAL,
  group_name VARCHAR NOT NULL,
  PRIMARY KEY(group_id)
);

CREATE TABLE history_log (
  history_id SERIAL,
  time_stamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  group_id INT NOT NULL,
  package_id INT NOT NULL,
  request_id INT NOT NULL,
  PRIMARY KEY(history_id)
);

CREATE TABLE registry (
  registry_id SERIAL,
  group_id INT NOT NULL,
  package_id INT NOT NULL,
  user_id INT NOT NULL,
  PRIMARY KEY(registry_id)
);

ALTER TABLE history_log
    ADD CONSTRAINT history_log_group_id 
    FOREIGN KEY (group_id) 
    REFERENCES groups(group_id) 
    ON UPDATE CASCADE ON DELETE RESTRICT;

ALTER TABLE history_log
    ADD CONSTRAINT history_log_package_id 
    FOREIGN KEY (package_id) 
    REFERENCES packages(package_id) 
    ON UPDATE CASCADE ON DELETE RESTRICT;

ALTER TABLE history_log
    ADD CONSTRAINT history_log_request_id 
    FOREIGN KEY (request_id) 
    REFERENCES requests(request_id) 
    ON UPDATE CASCADE ON DELETE RESTRICT;

ALTER TABLE registry
    ADD CONSTRAINT registry_group_id 
    FOREIGN KEY (group_id) 
    REFERENCES groups(group_id) 
    ON UPDATE CASCADE ON DELETE RESTRICT;

ALTER TABLE registry
    ADD CONSTRAINT registry_package_id 
    FOREIGN KEY (package_id) 
    REFERENCES packages(package_id) 
    ON UPDATE CASCADE ON DELETE RESTRICT;

ALTER TABLE registry
    ADD CONSTRAINT registry_user_id 
    FOREIGN KEY (user_id) 
    REFERENCES users(user_id) 
    ON UPDATE CASCADE ON DELETE RESTRICT;