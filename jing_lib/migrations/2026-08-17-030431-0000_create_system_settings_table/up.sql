-- Your SQL goes here
CREATE TABLE system_settings (
    id SERIAL PRIMARY KEY,
    setting_section VARCHAR(255) NOT NULL,
    setting_key VARCHAR(255) NOT NULL UNIQUE,
    setting_value VARCHAR(255) NOT NULL
);

INSERT INTO system_settings (setting_section, setting_key, setting_value) VALUES
('smtp', 'smtp_username', 'example@example.com'),
('smtp', 'smtp_password', 'password123'),
('smtp', 'smtp_host', 'smtp.example.com'),
('smtp', 'smtp_port', '587'),
('smtp', 'smtp_use_tls', 'true'),
('smtp', 'smtp_use_ssl', 'false'),
('admin', 'admin_email', 'admin@example.com');
