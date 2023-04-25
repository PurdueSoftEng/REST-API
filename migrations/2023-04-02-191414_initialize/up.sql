CREATE TABLE packages (
  package_id SERIAL PRIMARY KEY,
  link VARCHAR NOT NULL,
  package_name VARCHAR NOT NULL,
  metric_one INT NOT NULL,
  metric_two INT NOT NULL,
  metric_three INT NOT NULL,
  metric_four INT NOT NULL,
  metric_five INT NOT NULL,
  metric_six INT NOT NULL,
  metric_seven INT NOT NULL,
  total_score INT NOT NULL
);

CREATE TABLE users (
  user_id SERIAL PRIMARY KEY,
  user_name VARCHAR NOT NULL
);

CREATE TABLE requests (
  request_id SERIAL PRIMARY KEY,
  request_type VARCHAR NOT NULL
);

CREATE TABLE groups (
  group_id SERIAL PRIMARY KEY,
  group_name VARCHAR NOT NULL
);

CREATE TABLE history_log (
  history_id SERIAL PRIMARY KEY,
  time_stamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  group_id INT NOT NULL,
  package_id INT NOT NULL
);

CREATE TABLE registry (
  registry_id SERIAL PRIMARY KEY,
  request_id INT NOT NULL,
  package_id INT NOT NULL,
  user_id INT NOT NULL
);





