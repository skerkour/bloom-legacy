/* eslint-disable import/prefer-default-export */

export enum Method {
  FetchPlans = 'billing.fetchPlans',
  DeletePlan = 'billing.deletePlan',
  UpdatedPlan = 'billing.updatePlan',
  CreatePlan = 'billing.createPlan',
  FetchMyProfile = 'billing.fetchMyProfile',
  AddPaymentMethod = 'billing.addPayment_method',
  RemovePaymentMethod = 'billing.removePaymentMethod',
  UpdateSubscription = 'billing.updateSubscription',
}

export type NewStripeCard = {
  number: string,
  expMonth: string,
  expYear: string,
  cvc: string,
}

export type AppPaymentMethodParams= {
  stripePublicKey: string | null,
  groupId: string | null,
  card: NewStripeCard,
}
