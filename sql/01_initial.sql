use nacos_config;
INSERT INTO users (username, password, enabled)
VALUES (
    'nacos',
    '$2a$10$EuWPZHzz32dJN7jexM34MOeYirDdFAZm2kuWj7VEOJhhZkDrxfvUu',
    TRUE
  );
INSERT INTO roles (username, role)
VALUES ('nacos', 'ROLE_ADMIN');
use forum;
INSERT INTO `t_srp_password` (
    `identifier`,
    `verifier`,
    `salt`,
  )
VALUES (
    'heliannuuthus@gmail.com',
    'cd5710ca00d140994f2fe14f26ecf051e3548b7d493c8ea93d73de008fa8e960',
    '608f784b6114f5ade421222838e25938d7d0872de47c8831d9d32c124d1840ff1ab1706b42645dc4',
);
