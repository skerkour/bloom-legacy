CREATE TABLE billing_plans (
    id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,

    name TEXT NOT NULL,
    price BIGINT NOT NULL,
    description TEXT NOT NULL,
    stripe_id TEXT NOT NULL,
    is_active BOOLEAN NOT NULL,
    product TEXT NOT NULL,
    storage BIGINT NOT NULL,

    PRIMARY KEY(id)
);

INSERT INTO billing_plans
		(id, created_at, updated_at, name, description, stripe_id, price, is_active, product, storage)
		VALUES ('42fb1c42-caca-418d-81f3-a6313c4a0a42', '2020-02-10T14:33:39+00:00', '2020-02-10T14:33:39+00:00',
    'Free', '', 'plan_Gck0Zy9Qx5qaGk', 0, true, 'FREE', 1000000000); -- 1GB

CREATE TABLE billing_customers (
    id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,

    email TEXT NOT NULL,
    stripe_customer_id TEXT,
    stripe_subscription_id TEXT,
    used_storage BIGINT NOT NULL,
    subscription_updated_at TIMESTAMP WITH TIME ZONE NOT NULL,

    plan_id UUID NOT NULL REFERENCES billing_plans(id),
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
