INSERT INTO users (
    id,
    username,
    email,
    password_hash,
    role,
    password_changed_at
) VALUES (
    UUID(),
    'admin',
    'admin@catalystrsx.dev',
    '$argon2id$v=19$m=19456,t=2,p=1$c29tZXNhbHQ$2hPBWK5jNYQ5yqwHN7/BHzX3gcz39EqVYC06LyyXBnw',
    'admin',
    CURRENT_TIMESTAMP
);
