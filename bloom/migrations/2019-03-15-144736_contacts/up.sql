-- Your SQL goes here
CREATE TABLE contacts_contacts (
    id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
    deleted_at TIMESTAMP WITH TIME ZONE,
    version BIGINT NOT NULL,

    addresses JSONB[] NOT NULL,
    birthday TIMESTAMP WITH TIME ZONE,
    company TEXT,
    emails JSONB[] NOT NULL,
    first_name TEXT,
    last_name TEXT,
    notes TEXT,
    occupation TEXT,
    organizations JSONB[] NOT NULL,
    phones JSONB[] NOT NULL,
    websites JSONB[] NOT NULL,

    owner_id UUID NOT NULL REFERENCES account_accounts (id),

    PRIMARY KEY(id)
);

CREATE TABLE contacts_contacts_events (
    id UUID NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
    aggregate_id UUID NOT NULL REFERENCES contacts_contacts (id),
    data JSONB NOT NULL,
    metadata JSONB NOT NULL,

    PRIMARY KEY(id)
);
