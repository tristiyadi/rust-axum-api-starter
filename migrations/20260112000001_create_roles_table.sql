-- Create roles table
DROP TABLE IF EXISTS `roles`;
CREATE TABLE `roles` (
  `roles_id` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(256) NOT NULL,
  `display_name` varchar(256) NOT NULL,
  `description` text DEFAULT NULL,
  `status` tinyint(1) NOT NULL DEFAULT 0,
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  PRIMARY KEY (`roles_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Seed Administrator role
INSERT INTO `roles` (`roles_id`, `name`, `display_name`, `description`, `status`, `created_at`, `updated_at`) 
VALUES (1, 'Administrator', 'Administrator', 'Full akses', 1, NOW(), NOW());
INSERT INTO `roles` VALUES (2, 'User', 'User', 'Half akses', 1, NOW(), NOW());
