-- Update users table with new schema
DROP TABLE IF EXISTS `users`;
CREATE TABLE `users` (
  `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(191) NOT NULL,
  `email` varchar(191) NOT NULL,
  `email_verified_at` datetime DEFAULT NULL,
  `password` varchar(191) DEFAULT NULL,
  `status` varchar(10) DEFAULT NULL,
  `uid` varchar(191) DEFAULT NULL,
  `role_id` int(10) unsigned NOT NULL DEFAULT 2,
  `username` varchar(200) DEFAULT NULL,
  `remember_token` varchar(100) DEFAULT NULL,
  `users_token` varchar(255) DEFAULT NULL,
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`),
  UNIQUE KEY `users_email_unique` (`email`),
  UNIQUE KEY `users_users_token_unique` (`users_token`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Seed Administrator user (Password: password)
INSERT INTO `users` (`name`, `email`, `password`, `status`, `uid`, `role_id`, `username`, `created_at`, `updated_at`) 
VALUES ('Administrator', 'eko@trimogo.com', '$2y$10$92IXUNpkjO0rOQ5byMi.Ye4oKoEa3Ro9llC/.og/at2.uheWG/igi', 'active', '550e8400-e29b-41d4-a716-446655440000', 1, 'eko', NOW(), NOW());
