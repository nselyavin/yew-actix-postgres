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
backed:
```
  cd backend
  cargo run --bin backend 
```
frontend:
```
  cd frontend
  trunk serve
```
____
## Api
##### localhost:8080/
| Type          | Request         | Description  |
|:------------- |:----------------| -------------:|
| GET           | /               | Entrypoint of the app |
| GET           | /users          |          Get all users|
| GET           | /users/{id}     |         Get user by id|
| POST          | /signup         |         User Signup|
| POST          | /login          |         User Login|
| GET           | /medicines       |         Get all medicines|
| GET           | /medicines/{id}  |         Get medicine by id|
| POST           | /medicines/{id}/add  |         Add medicine to user bucket|
