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
  FetchGroupProfile = 'billing.fetchGroupProfile',
}

export type NewStripeCard = {
  number: string,
  expMonth: string,
  expYear: string,
  cvc: string,
}

export type AppPaymentMethodParams= {
  stripePublicKey: string | null,
  groupID: string | null,
  card: NewStripeCard,
}

export type FetchGroupProfileParams = {
  id: string,
}
