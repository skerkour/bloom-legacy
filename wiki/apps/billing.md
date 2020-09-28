# Billing


1. [Overview](#overview)
2. [Scenarios](#scenarios)
3. [Non goals](#non-goals)
4. [Models](#models)
5. [API](#api)
6. [Views](#views)

-------------------


## Overview


* See https://stripe.com/docs/billing/lifecycle

## Scenarios

The Billing service is used to charge customer for service provided by us.

In order to stay privacy oriented, but secure, we need to use external providers (Stripe, Paypal) to store customers card details, but to not send personal information to those providers, and use an anonymous customer id.

## Non goals

* Crypto payments

## Notes

on cree le customer stripe en meme temps que le compte, ou au moment du premier paiement ?
-> au moment du premier paiement => moins de dependance a stripe

comment on affiche un message en cas d'invoice non payée ?
 -> avoir un champ dans `users.Me`


## Models

### Customer

A `Customer` is used to store general purpose data about the billing profile of the user or the group,
like the legally required information.

```go
type Customer struct {
    ID string
    CreatedAt time.Time
    UpdatedAt time.Time

    StripeAnonymousID string // an UUID generated API side to anonymize the user / group
    StripeCustomerID string

    Storage int64
    ParallelBitflowDownloads int64

    UserID *string
    GroupID *string
}
```

#### Commands

##### Create

 create the customer, at the same time of the user / group creation

#### Validators

### Plan

A plan is used to store the allowed features and volume per price

```go
type Plan struct {
    ID string
    CreatedAt time.Time
    UpdatedAt time.Time

    Name string
    DriveStorage int64
    ParallelBitflowDownloads int64
}
```

### Invoice

Invoices must embed all the data legally required, like the products (references?),
the quantity, the price, the invoice unique number...

```go
type Invoice struct {
    ID string
    CreatedAt time.Time
    UpdatedAt time.Time

    SubscriptionID string
}
```

#### Commands

#### Validators


### Payement Method

Payment methods are used to charge money to customers, it can be a credit card,
a PayPal account, a Apple Pay account...

```go
type PaymentMethod struct {
    ID string
    CreatedAt time.Time
    UpdatedAt time.Time

    SubscriptionID string
}
```

#### Commands

#### Validators


### Subscription

A subscription is used to allow recurring billing

```rust
pub struct Subscription {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub version: i64,

    pub billing_profile_id: uuid::Uuid,
}
```

#### Commands

#### Validators


## API

### Routes

### Models



## Views



## Resources

* https://stripe.com/docs/billing/quickstart
* https://news.ycombinator.com/item?id=19608140
* https://news.ycombinator.com/item?id=19473336
* https://blog.checklyhq.com/managing-eu-vat-with-stripe-for-saas-is-not-that-hard/
