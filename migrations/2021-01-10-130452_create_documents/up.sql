CREATE TABLE documents (
    id bigint unsigned NOT NULL AUTO_INCREMENT,
    title varchar(500) NOT NULL,
    body text NOT NULL,
    PRIMARY KEY(`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
