ALTER TABLE billing_plans RENAME COLUMN is_active TO is_public;

CREATE UNIQUE INDEX billing_plans_stripe_id_idx ON billing_plans (stripe_id);
CREATE INDEX billing_customers_user_id_idx ON billing_customers (user_id);
CREATE INDEX billing_customers_group_id_idx ON billing_customers (group_id);
CREATE INDEX billing_customers_plan_id_idx ON billing_customers (plan_id);
CREATE INDEX billing_payment_methods_customer_id_idx ON billing_payment_methods (customer_id);
