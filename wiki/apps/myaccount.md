# MyAccount


1. [Overview](#overview)
2. [Scenarios](#scenarios)
3. [Non goals](#non-goals)
4. [Models](#models)
5. [Views](#views)
6. [Open issues](#open-issues)

-------------------


## Overview

The purpose of the `Account` service is to create an account, signin and update an user's account data.

The `Account` service is the only service which handle this tasks, so in order to authenticate other services it should be able to
trasmit the session data to other services and redirect to other services.


## Scenarios

### 1
Marina wants to use bloom drive. To do so she needs to create an account. So on `drive.bloom.sh` she click on `Register`.
She is redirected to `account.bloom.sh` to the registration form.

She fills the forms, receive the email and choose her username where she see in realtime if her username is valid.

Once the registration completed she is redirected to `drive.bloom.sh`.

### 2
Sylvain wants to use `cloud.bloom.sh` but is signed out. He clicks on the `Sign in` button and is redirected to `account.bloom.sh` to the sign in form.
After a succssful sign in he is redirected to `cloud.bloom.sh`.


### 3
Zinedine wanst to change his account avatar. He clicks on the `Account` button and is redirected to `account.bloom.sh`.
Here he sees his account profile and update it.


## Non goals

This version will **not** support the following features:
- username update
- email update
- billing


## Models


### Account

`first_name`
- can't be empty

`last_name`
- can't be empty

`email`
- must be lowercased
- can't be an anonymous email domain


#### Email update flow
1. the user enter the new email in it's profile account
2. it verify that the email is not already in use with another account
3. it create a new pending email associated with the account
4. it send an email to the adress with a verification code
5. the user enter the verification code and verify the email
6. delete all the currently pending email for the account, and all pending emails with this email, and replace the current account email
7. send a security email to the previous email


`password`
- 8 to 257 characters
- can't be email
- can't be `first_name` + `last_name`

`username`
- 4 to 32
- a-z, 0-9 and . (case sensitive)
- can't start or end with a .
- can't have 2 consecutives .
- should be a valid first part of email

`avatar`
- Max 3MB
- JPEG or PNG
- cropped and resized to 256 * 256
- converted to JPEG

### Session

## Views


### Sign in

path: `/signin`

path: `/login` -> redirect to `/signin`

there is only 2 fields: `username` and `password` and 1 button: `Sign in`

When the user click the `Sign in` button or hit the `enter` in the password field, a request is submitted to the API.

if the pair is not valid (account with username does not exist or password is bad):
`Invalid username/password combination`

if the pair is valid:
- if a query parameter `service` is given with the name of an existing service, redirect to the service
- else redirect to `/`

send an email alert

### Register

path: `/register`

path: `/signup` -> redirect to `/register`

path: `**` -> redirect to `/register`

fields:
- `First name`
- `Last name`
- `Email`
- `Password`

when a key is pressed in the `Email` field an automated API call is performed to check if the email is valid or already taken
- if it's invalid the validator message with the violated rule is displayed
- if it's already taken the following message is displayed: `Email is already taken`

send a verification code email

### Welcome, code verification
path: `/welcome/verify`

there is a button to resend email

when the code is entered an 'automated' API call is performed

if the code is valid:
- redirect to `/welcome/username`
- else if the code has expired display an error message: `Confirmation code has expired`
- else `Confirmation code is not valid`

### Welcome, choose username
path: `/welcome/username`

fields:
- `Username`

when a key is pressed in the `Username` field an automated API call is performed to check if the username is valid or already taken
- if it's invalid the validator message with the violated rule is displayed
- if it's already taken following message is displayed: `Username already taken`

send a welcome email (delayed ?) with information on the platform

### Account recovery, request
path: `/recovery/request`

fields:
- `Email or username`

send an email with a link with an unique (id, token) pair


### Account recovery, update password
path: `/recovery`

fields:
- `New password`
- `Verify password`


send an email alert when completed


### Profile
path: `/account`

ici on peut acceder aux info de son comptes et les modifier:
- `avatar`
- `first_name`
- `last_name`
- `email`

et voir son `username`


### Security
path: `/account/security`

fields:
- `Current password`
- `New password`
- `Verify password`

ici on peut modifier son `password` (necessite l'actuel)

et plus tard gerer la 2FA

### Session
path: `/account/sessions`


ici on peut voir ses `sessions` actives et revoker n'importe laquelle sauf l'actuelle

`events` where `metadata.actor_id` == `account.id`

### Audit log
path: `/account/audit`


ici on peut voir tous les events en rapport avec notre compte

## Open issues

- Do we indicate that email is used only for recovery (because it's also a contact email).
- is the welcome email delayed ?

- Biling
- username update
- email update
