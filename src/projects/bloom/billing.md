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


## Models

### Profile

The Billing profile is used to store general purpose data about the customer,
like the legally required information.

```rust
pub struct Profile {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub version: i64,

    pub strip_anonymous_id: String,
    pub stripe_customer_id: Option<String>,

    pub account_id: uuid::Uuid,
}
```

#### Commands

#### Validators


### Invoice

Invoices must embed all the data legally required, like the products (references?),
the quantity, the price, the invoice unique number...

```rust
pub struct Invoice {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub version: i64,

    pub billing_profile_id: uuid::Uuid,
}
```

#### Commands

#### Validators


### Payement Method

Payment methods are used to cherge money to customers, it can be a credit card,
a PayPal account, a Apple Pay account...

```rust
pub struct PaymentMethod {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub version: i64,

    pub details: PaymentDetails,
    pub is_default: bool,

    pub billing_profile_id: uuid::Uuid,
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



-------------------------------

* https://stripe.com/docs/billing/quickstart
* https://news.ycombinator.com/item?id=19608140
* https://news.ycombinator.com/item?id=19473336
* https://blog.checklyhq.com/managing-eu-vat-with-stripe-for-saas-is-not-that-hard/
* https://forwardemail.net/#/
* https://news.ycombinator.com/item?id=15405968
* https://note.delivery/#/
* https://cryptsend.io/
* https://www.msgsafe.io/
* https://grove.io/
* https://crypton.sh/
* https://news.ycombinator.com/item?id=18128841
* https://box.matto.nl/encryptedremotegit.html
* https://news.ycombinator.com/item?id=15401211
* http://ptspts.blogspot.com/2017/10/comparison-of-encrypted-git-remote.html
* https://www.agwa.name/projects/git-crypt/
* https://git-secret.io/
* https://www.cyph.com/
* https://blog.cryptographyengineering.com/
* https://www.cyph.com/castle
* https://blog.whatsapp.com/
* https://blog.trailofbits.com/2018/10/22/a-guide-to-post-quantum-cryptography/
* https://news.ycombinator.com/item?id=18280572
* https://news.ycombinator.com/item?id=18259393
* https://core.telegram.org/methods
* http://unhandledexpression.com/crypto/security/2013/12/20/criterions-for-a-crypto-app.html
* https://eprint.iacr.org/2018/1121
