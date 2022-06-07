# Pharmacy-app
____
## Preparing
To start the application Rust compiler and Cargo build manager must be installed https://www.rust-lang.org/learn/get-started

```
  cargo install trunk
  rustup target add wasm32-unknown-unknown
  cargo install --locked trunk
  cargo install wasm-bindgen-cli
  
  git clone git@github.com:Fume5678/pharmacy-app.git
  cd pharmacy-app
```
## Deploy

The server requires a running PostgreSQL server. The backend/migration folder contains up.sql file, that setup database
```
  psql -U postgres -f up.sql
```
It also create a test user:\
name:     test\
email:    test@test.ru\
password: 1324

In the .env file you can point database_url "postgres://user:password@localhost:5432/pharmacy"\
Start backend server:
```
  cd backend
  cargo run --bin backend 
```
frontend:
```
  cd frontend
  trunk serve --port 3000
```
____
## Frontend uri
##### http://localhost:3000

## Backend Api
##### http://localhost:8080/
### ***REST API documentation:***
### */info/{id}



***Endpoint:***

```bash
Method: GET
Type: 
URL: localhost:8080/info/b49fbf88-43ea-4aaa-bb2e-74897979d013
```



### */login



***Endpoint:***

```bash
Method: POST
Type: RAW
URL: localhost:8080/login
```



***Body:***

```js        
{
    "email": "nselyavin@inbox.ru",
    "password": "1234"
}
```



### */signup



***Endpoint:***

```bash
Method: POST
Type: RAW
URL: localhost:8080/signup
```



***Body:***

```js        
{
    "email": "test1234@test.ru",
    "username": "testn123ame",
    "password": "1324"
}
```


### */medicine/new



***Endpoint:***

```bash
Method: POST
Type: RAW
URL: localhost:8080/medicine/new
Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6NjI1MzE5ODUwMzczNzE2Nzg3MiwiZXhwIjoxNjUzOTExODI2fQ.UnkhAp5_jEEW52jUs_opnsAH8YoGPX7-UXjC4YZTkC8
```



***Body:***

```js        
{
    "name": "name",
    "description": "description",
    "cost": 1000.1
}

```


### */user/detail



***Endpoint:***

```bash
Method: GET
Type: 
URL: localhost:8080/user/detail
Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6NjI1MzE5ODUwMzczNzE2Nzg3MiwiZXhwIjoxNjUzOTExODI2fQ.UnkhAp5_jEEW52jUs_opnsAH8YoGPX7-UXjC4YZTkC8
```




