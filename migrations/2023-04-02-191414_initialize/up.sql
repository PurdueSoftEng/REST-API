CREATE TABLE `app`.`packages` (
  `package_id` INT SERIAL AUTO_INCREMENT,
  `view_time` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `url` VARCHAR(2048) NOT NULL,
  `package_name` VARCHAR(1024) NOT NULL,
  `metric_one` INT NOT NULL,
  `metric_two` INT NOT NULL,
  `metric_three` INT NOT NULL,
  `metric_four` INT NOT NULL,
  `metric_five` INT NOT NULL,
  `metric_six` INT NOT NULL,
  `metric_seven` INT NOT NULL,
  `total_score` INT NOT NULL,
  PRIMARY KEY (`package_id`)
);

CREATE TABLE `app`.`user` (
  `user_id` INT SERIAL AUTO_INCREMENT,
  `user_name` VARCHAR(1024) NOT NULL,
  PRIMARY KEY (`user_id`)
);

CREATE TABLE `app`.`request` (
  `request_id` INT SERIAL AUTO_INCREMENT,
  `request_type` VARCHAR(1024) NOT NULL,
  PRIMARY KEY (`request_id`)
);

CREATE TABLE `app`.`group` (
  `group_id` INT SERIAL AUTO_INCREMENT,
  `group_name` VARCHAR(1024) NOT NULL,
  PRIMARY KEY (`group_id`)
);

CREATE TABLE `app`.`history_log` (
  `history_id` INT SERIAL AUTO_INCREMENT,
  `group_id` INT NOT NULL,
  `package_id` INT NOT NULL,
  PRIMARY KEY (`history_id`)
);

CREATE TABLE `app`.`registry` (
  `registry_id` INT SERIAL AUTO_INCREMENT,
  `request_id` INT NOT NULL,
  `package_id` INT NOT NULL,
  `user_id` INT NOT NULL,
  PRIMARY KEY (`registry_id`)
);




