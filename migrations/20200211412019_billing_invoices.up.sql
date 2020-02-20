CREATE TABLE billing_invoices (
    id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,

    amount BIGINT NOT NULL,
    stripe_id TEXT NOT NULL UNIQUE,
    stripe_hosted_url TEXT NOT NULL,
    stripe_pdf_url TEXT NOT NULL,
    paid BOOLEAN NOT NULL,

    customer_id UUID NOT NULL REFERENCES billing_customers(id),

    PRIMARY KEY(id)
);

CREATE INDEX billing_invoices_customer_id_idx ON billing_invoices (customer_id);

CREATE INDEX billing_customers_stripe_customer_id_idx ON billing_customers (stripe_customer_id);
