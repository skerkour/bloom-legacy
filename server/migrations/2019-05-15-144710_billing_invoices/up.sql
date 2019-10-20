-- Your SQL goes here
CREATE TABLE billing_invoices (
    id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
    deleted_at TIMESTAMP WITH TIME ZONE,
    version BIGINT NOT NULL,

    details JSONB NOT NULL,

    billing_profile_id UUID NOT NULL REFERENCES billing_profiles (id),

    PRIMARY KEY(id)
);


CREATE TABLE billing_invoices_events (
    id UUID NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
    aggregate_id UUID NOT NULL REFERENCES billing_invoices (id),
    data JSONB NOT NULL,
    metadata JSONB NOT NULL,

    PRIMARY KEY(id)
);
