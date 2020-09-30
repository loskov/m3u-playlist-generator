CREATE TABLE `category` (
    `id` INT(11) NOT NULL AUTO_INCREMENT,
    `name` VARCHAR(255) COLLATE utf8mb4_unicode_ci NOT NULL,
    `short_name` VARCHAR(255) COLLATE utf8mb4_unicode_ci NOT NULL,
    `is_active` TINYINT(1) NOT NULL,
    PRIMARY KEY (`id`),
    UNIQUE KEY `uniq_name` (`name`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE `channel` (
    `id` INT(11) NOT NULL AUTO_INCREMENT,
    `name` VARCHAR(255) COLLATE utf8mb4_unicode_ci NOT NULL,
    `tv_guide_logo` VARCHAR(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
    PRIMARY KEY (`id`),
    UNIQUE KEY `uniq_name` (`name`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE `channel_category` (
    `channel_id` INT(11) NOT NULL,
    `category_id` INT(11) NOT NULL,
    PRIMARY KEY (`channel_id`,`category_id`),
    KEY `idx_channel_id` (`channel_id`),
    KEY `idx_category_id` (`category_id`),
    CONSTRAINT `fk_channel_category_category_category_id` FOREIGN KEY (`category_id`) REFERENCES `category` (`id`) ON DELETE CASCADE,
    CONSTRAINT `fk_channel_category_channel_channel_id` FOREIGN KEY (`channel_id`) REFERENCES `channel` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE `source` (
    `id` INT(11) NOT NULL AUTO_INCREMENT,
    `name` VARCHAR(255) COLLATE utf8mb4_unicode_ci NOT NULL,
    `user_agent` VARCHAR(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
    `description` VARCHAR(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
    `is_active` TINYINT(1) NOT NULL,
    `query_string` VARCHAR(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
    PRIMARY KEY (`id`),
    UNIQUE KEY `uniq_name` (`name`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE `source_channel` (
    `id` INT(11) NOT NULL AUTO_INCREMENT,
    `source_id` INT(11) NOT NULL,
    `channel_id` INT(11) NOT NULL,
    `url` VARCHAR(255) COLLATE utf8mb4_unicode_ci NOT NULL,
    `is_active` TINYINT(1) NOT NULL,
    PRIMARY KEY (`id`),
    UNIQUE KEY `uniq_url` (`url`),
    KEY `idx_source_id` (`source_id`),
    KEY `idx_channel_id` (`channel_id`),
    CONSTRAINT `fk_source_channel_channel_channel_id` FOREIGN KEY (`channel_id`) REFERENCES `channel` (`id`),
    CONSTRAINT `fk_source_channel_source_source_id` FOREIGN KEY (`source_id`) REFERENCES `source` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE `tv_guide` (
    `id` INT(11) NOT NULL AUTO_INCREMENT,
    `name` VARCHAR(255) COLLATE utf8mb4_unicode_ci NOT NULL,
    `url` VARCHAR(255) COLLATE utf8mb4_unicode_ci NOT NULL,
    PRIMARY KEY (`id`),
    UNIQUE KEY `uniq_name` (`name`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE `tv_guide_channel` (
    `id` INT(11) NOT NULL AUTO_INCREMENT,
    `tv_guide_id` INT(11) NOT NULL,
    `channel_id` INT(11) NOT NULL,
    `external_id` VARCHAR(255) COLLATE utf8mb4_unicode_ci NOT NULL,
    PRIMARY KEY (`id`),
    KEY `idx_tv_guide_id` (`tv_guide_id`),
    KEY `idx_channel_id` (`channel_id`),
    CONSTRAINT `fk_tv_guide_channel_channel_channel_id` FOREIGN KEY (`channel_id`) REFERENCES `channel` (`id`),
    CONSTRAINT `fk_tv_guide_channel_tv_guide_tv_guide_id` FOREIGN KEY (`tv_guide_id`) REFERENCES `tv_guide` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
