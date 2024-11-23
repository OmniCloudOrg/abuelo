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
    "creation" : String,
    "is_premium" : Boolean,
}
```
- **success**: if the user is found successfully then the value returned is
true
- **message**: if success is false, contains an error message to give to the 
user
- **creation**: if success is true, contains the creation date of the account in the format
YYYY-MM-DD HH:MM
- **is_premium**: if succuss is true, contains whether or not the account is premium

## POST /user/:username/update
Updates the records in the database
Request Format:
```json
{
    "username" : String?,
    "password" : String,
    "new_password" : String?
}
```
(question marks indicate the value is nullable)
- **username**: The new username of the user
- **password**: The (plain-text currently but in future RSA encrypted) password of the user
- **new_password**: The new (plain-text currently but in future RSA encrypted) password of the user

Response Format:
```json
{
    "success" : Boolean,
    "message" : String,
}
```
- **success**: if the user is updated successfully then the value returned is
true
- **message**: if success is false, contains an error message to give to the user


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
}
```
- **success**: if the user is authed successfully then the value returned is
true
- **message**: if success is false, contains an error message to give to the user
