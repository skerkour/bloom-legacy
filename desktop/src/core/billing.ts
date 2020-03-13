/* eslint-disable import/prefer-default-export */

export enum Method {
  FetchPlans = 'billing.fetchPlans',
  DeletePlan = 'billing.deletePlan',
  UpdatedPlan = 'billing.updatePlan',
  CreatePlan = 'billing.createPlan',
  FetchMyProfile = 'billing.fetchMyProfile',
  AddPaymentMethod = 'billing.addPaymentMethod',
  RemovePaymentMethod = 'billing.removePaymentMethod',
  UpdateSubscription = 'billing.updateSubscription',
  ChangeDefaultPaymentMethod = 'billing.changeDefaultPaymentMethod',
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
