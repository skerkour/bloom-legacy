CREATE TABLE billing_plans (
    id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,

    name TEXT NOT NULL,
    price DOUBLE PRECISION NOT NULL,
    description TEXT NOT NULL,
    stripe_id TEXT NOT NULL,
    is_active BOOLEAN NOT NULL,
    tier TEXT NOT NULL,

    PRIMARY KEY(id)
);

CREATE TABLE billing_customers (
    id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,

    stripe_id TEXT,

    plan_id UUID NOT NULL REFERENCES billing_plans(id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    group_id UUID REFERENCES groups(id) ON DELETE CASCADE,

    PRIMARY KEY(id)
);

CREATE TABLE billing_payment_methods (
    id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,

    is_default BOOLEAN NOT NULL,
    stripe_id TEXT NOT NULL,
    card_last_4 TEXT NOT NULL,
    card_expiration_month BIGINT NOT NULL,
    card_expiration_year BIGINT NOT NULL,

    customer_id UUID NOT NULL REFERENCES billing_customers(id) ON DELETE CASCADE,

    PRIMARY KEY(id)
);

CREATE TABLE billing_invoices (
    id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,

    stripe_id TEXT NOT NULL,

    customer_id UUID NOT NULL REFERENCES billing_customers(id) ON DELETE CASCADE,

    PRIMARY KEY(id)
);
