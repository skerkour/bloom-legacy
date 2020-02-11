ALTER TABLE billing_plans RENAME COLUMN is_public TO is_active;

DROP INDEX IF EXISTS billing_plans_stripe_id_idx;
DROP INDEX IF EXISTS billing_customers_user_id_idx;
DROP INDEX IF EXISTS billing_customers_group_id_idx;
DROP INDEX IF EXISTS billing_customers_plan_id_idx;
DROP INDEX IF EXISTS billing_payment_methods_customer_id_idx;
DROP INDEX IF EXISTS billing_invoices_customer_id_idx;
