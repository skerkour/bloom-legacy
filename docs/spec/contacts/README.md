# Contacts

1. [Overviw](#overviw)
2. [Scenarios](#scenarios)
3. [Non goals](#non-goals)
4. [Models](#models)
5. [Views](#views)

-------------------


## Overview

Contacts is a simple application allowing users to manage their contacts

#TODOS;
- labels

## Scenarios

### 1
Sylvain does not wnat to have to remember the phone number of all his family. He uses contacts to store
the names, phone numbers, Bloom usernames, website and other information about his friends.



## Non goals

- messaging app



## Models

### Contact

- first_name
- last_name
- company
- occupation
- address ()
- birthday
- website
- notes
- emails []string
- phones { prefix (country), phone, label: home|mobile|work|other|main|Home Fax|Work fax}


#### Events
- created
- deleted
- first_name_updated
- last_name_updated
- company_updated
- occupation_updated
- address_updated
- birthday_updated
- notes_udpated
- emails_updated
- phones_updated
- website_updated


### Views

contacts
create/view/edit dialog

searchbar (frontend only pour commencer)

