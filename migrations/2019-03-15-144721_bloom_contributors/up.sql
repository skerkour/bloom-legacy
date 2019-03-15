-- Your SQL goes here
CREATE TABLE bloom_contributors (
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
    deleted_at TIMESTAMP WITH TIME ZONE,

    github_login TEXT NOT NULL,
    commits BIGINT NOT NULL,

    PRIMARY KEY(github_login)
);
