/* eslint-disable import/prefer-default-export, camelcase */

enum Method {
  FetchPlans = 'billing.fetch_plans',
  DeletePlan = 'billing.delete_plan',
  UpdatedPlan = 'billing.update_plan',
  CreatePlan = 'billing.create_plan',
  FetchMyProfile = 'billing.fetch_my_profile',
  AddPaymentMethod = 'billing.add_payment_method',
  RemovePaymentMethod = 'billing.remove_payment_method',
  UpdateSubscription = 'billing.update_subscription',
}

export { Method };


export type NewStripeCard = {
  number: string,
  exp_month: string,
  exp_year: string,
  cvc: string,
}

export type AppPaymentMethodParams= {
  stripePublicKey: string | null,
  groupId: string | null,
  card: NewStripeCard,
}
