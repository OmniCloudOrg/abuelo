# ABUELO
Abuelo is an open source profile service meant to organize development around 
AbleOS and related projects. The rest of this document is documentation for 
the API


## GET /abuelo/user/:username
Return information about a particular user in the following format:
```json
{
    "success" : Boolean,
    "message" : String,
    "username" : String,
    "email" : String
}
```
- **success**: if the user is found successfully then the value returned is
true
- **message**: if success is false, contains an error message to give to the 
user
- **username**:
