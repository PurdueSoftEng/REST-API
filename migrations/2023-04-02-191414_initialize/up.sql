CREATE TABLE `app`.`packages` (
  `package_id` BIGINT NOT NULL AUTO_INCREMENT,
  `url` VARCHAR(2048) NOT NULL,
  `package_name` VARCHAR(1024) NOT NULL,
  `metric_one` TINYINT NOT NULL,
  `metric_two` TINYINT NOT NULL,
  `metric_three` TINYINT NOT NULL,
  `metric_four` TINYINT NOT NULL,
  `metric_five` TINYINT NOT NULL,
  `metric_six` TINYINT NOT NULL,
  `metric_seven` TINYINT NOT NULL,
  `total_score` TINYINT NOT NULL,
  PRIMARY KEY (`package_id`)
);

CREATE TABLE `app`.`users` (
  `user_id` INT NOT NULL AUTO_INCREMENT,
  `user_name` VARCHAR(1024) NOT NULL,
  PRIMARY KEY (`user_id`)
);

CREATE TABLE `app`.`requests` (
  `request_id` INT NOT NULL AUTO_INCREMENT,
  `request_type` VARCHAR(1024) NOT NULL,
  PRIMARY KEY (`request_id`)
);

CREATE TABLE `app`.`groups` (
  `group_id` INT NOT NULL AUTO_INCREMENT,
  `group_name` VARCHAR(1024) NOT NULL,
  PRIMARY KEY (`group_id`)
);

CREATE TABLE `app`.`history_log` (
  `history_id` INT NOT NULL AUTO_INCREMENT,
  `time` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `group_id` INT NOT NULL,
  `package_id` INT NOT NULL,
  PRIMARY KEY (`history_id`)
);

CREATE TABLE `app`.`registry` (
  `registry_id` INT NOT NULL AUTO_INCREMENT,
  `request_id` INT NOT NULL,
  `package_id` INT NOT NULL,
  `user_id` INT NOT NULL,
  PRIMARY KEY (`registry_id`)
);




