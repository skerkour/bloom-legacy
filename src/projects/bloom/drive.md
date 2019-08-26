# Drive

1. [Overviw](#overviw)
2. [Scenarios](#scenarios)

-------------------

## Overview

The drive service is a user friendly cloud storage solution.

An user should be able to upload files, create folders to organise his files, download files, rename files or folders


## Scenarios

### 1
Sylvain wants to send a file from his computer to his phone. he uplaods the file from his computer and can download it later from his phone.


### 2
Marina wants to backup her holidays pictures. She create a folder, She Select all her photos on her computer and upload it to her drive.

### 3
After having uploaded some files, zinedine wants to search amongs them. He uses the search bar.

## Non goals
- file convertion
- copies
- apps binding
- drag and drop
- downlaod multiple files
- file edition or reading
- file sharing
- billing
- starred


-is it possible to have files with same name in the same folder ? no

## Models
`file`

`drive_profile`
- space -> used space
- home -> home directory (file)

### initial folders
- Downloads
- Documents
- Music
- Pictures
- Books
- Videos
- Projects



## Views

### Sidebar
- + new button (or fab on mobile) -> Folder | File upload, Folder uplaod
- trash

### secondary toolbar
- delete
- move to
- Download

### new
- Folder
separator
- Files upload
- Folder upload

### Home
path: `/drive`

### Folder
path: `/drive/folders/{folder_id}`

### Trash
path: `/drive/trash`
