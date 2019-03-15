-- Your SQL goes here
CREATE TABLE contacts_contacts (
    id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
    deleted_at TIMESTAMP WITH TIME ZONE,
    version BIGINT NOT NULL,

    first_name TEXT,
    last_name TEXT,
    company TEXT,
    occupation TEXT,
    notes TEXT,
    organizations JSONB,
    websites JSONB,
    addresses JSONB,
    emails JSONB,
    phones JSONB,
    birthday TIMESTAMP WITH TIME ZONE,
    owner_id UUID NOT NULL REFERENCES account_accounts (id),

    PRIMARY KEY(id)
);

CREATE TABLE contacts_contacts_events (
    id UUID NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
    aggregate_id UUID NOT NULL,
    data JSONB NOT NULL,
    metadata JSONB NOT NULL,

    PRIMARY KEY(id)
);
