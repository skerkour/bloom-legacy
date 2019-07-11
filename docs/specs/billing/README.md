# Billing


1. [Overview](#overview)
2. [Scenarios](#scenarios)
3. [Non goals](#non-goals)
4. [Models](#models)
5. [API](#api)
6. [Views](#views)

-------------------


## Overview

## Scenarios


## Non goals




## Models

### Profile

```rust
pub struct Profile {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub version: i64,

    pub stripe_customer_id: Option<String>,

    pub account_id: uuid::Uuid,
}
```

#### Commands

#### Validators


### Invoice

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
