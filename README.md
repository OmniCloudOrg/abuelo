# ABUELO
Abuelo is an open source profile service meant to organize development around 
AbleOS and related projects. The rest of this document is documentation for 
the API


## GET /user/:username
Return information about a particular user in the following format:
```json
{
    "success" : Boolean,
    "message" : String,
    "creation_time" : String,
    "premium" : Boolean,
}
```
- **success**: if the user is found successfully then the value returned is
true
- **message**: if success is false, contains an error message to give to the 
user
- **creation_time**: if success is true, contains the creation date of the account in the format
YYYY-MM-DD HH:MM
- **premium**: if success is true, contains whether or not the account is premium

## POST /user/create
Adds a user to the database
Request Format:
```json
{
    "username" : String,
    "password" : String,
}
```
- **username**: The username of the newly created user
- **password**: The (plain-text currently but in future RSA encrypted) password of the newly created user

Response Format:
```json
{
    "success" : Boolean,
    "message" : String,
}
```
- **success**: if the user is created successfully then the value returned is
true
- **message**: if success is false, contains an error message to give to the user

## POST /user/auth
Authorizes the user
Request Format:
```json
{
    "username" : String,
    "password" : String,
}
```
- **username**: The username of the user
- **password**: The (plain-text currently but in future RSA encrypted) password of the user

Response Format:
```json
{
    "success" : Boolean,
    "message" : String,
    "handle": Number?
}
```
- **success**: if the user is authed successfully then the value returned is
true
- **message**: if success is false, contains an error message to give to the user
- **handle**: if success is true, contains a handle number for the user

## GET /user/:username/handles
Return all handles for a user
Response Format:
```json
{
    "success" : Boolean,
    "message" : String,
    "handles" : [Number]?
}
```
- **success**: if the handles are found successfully then the value returned is
true
- **message**: if success is false, contains an error message to give to the user
- **handles**: if success is true, contains an array of handle numbers belonging to the user

## POST /user/handle/create
Creates a new handle for a user
Request Format:
```json
{
    "username" : String,
    "password" : String,
}
```
- **username**: The username of the user
- **password**: The (plain-text currently but in future RSA encrypted) password of the user

Response Format:
```json
{
    "success" : Boolean,
    "message" : String,
    "handle" : Number?
}
```
- **success**: if the handle is created successfully then the value returned is
true
- **message**: if success is false, contains an error message to give to the user
- **handle**: if success is true, contains the newly created handle number

## POST /user/handle/delete
Deletes a handle from a user
Request Format:
```json
{
    "username" : String,
    "password" : String,
    "handle" : Number
}
```
- **username**: The username of the user
- **password**: The (plain-text currently but in future RSA encrypted) password of the user
- **handle**: The handle number to delete

Response Format:
```json
{
    "success" : Boolean,
    "message" : String,
    "handle" : Number?
}
```
- **success**: if the handle is deleted successfully then the value returned is
true
- **message**: if success is false, contains an error message to give to the user
- **handle**: if success is true, contains the deleted handle number