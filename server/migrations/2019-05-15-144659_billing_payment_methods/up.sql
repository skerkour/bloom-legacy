-- Your SQL goes here
CREATE TABLE billing_payment_methods (
    id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
    deleted_at TIMESTAMP WITH TIME ZONE,
    version BIGINT NOT NULL,

    details JSONB NOT NULL,
    is_default BOOLEAN NOT NULL,

    account_id UUID NOT NULL REFERENCES kernel_accounts (id),

    PRIMARY KEY(id)
);


CREATE TABLE billing_payment_methods_events (
    id UUID NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
    aggregate_id UUID NOT NULL REFERENCES billing_payment_methods (id),
    data JSONB NOT NULL,
    metadata JSONB NOT NULL,

    PRIMARY KEY(id)
);
