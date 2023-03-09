-- Add migration script here

CREATE TABLE IF NOT EXISTS user_profile(
    id INT AUTO_INCREMENT PRIMARY KEY NOT NULL COMMENT 'User ID',
    username VARCHAR(20) NOT NULL COMMENT 'Username',
    email VARCHAR(50) UNIQUE NOT NULL COMMENT 'Email',
    password VARCHAR(200) NOT NULL COMMENT 'Password',
    pro_level INT NOT NULL DEFAULT 0 COMMENT '会员等级: 0-普通用户,1-普通会员,2-超级会员',
    created_time TIMESTAMP NOT NULL COMMENT '注册时间',
    pro_end_time TIMESTAMP NOT NULL COMMENT '会员结束时间'
);