curl -X POST http://localhost:8000/user/auth \
 -H 'Content-Type: application/json' \
 -d '{"username" : "my_login", "password" : "my_password"}'