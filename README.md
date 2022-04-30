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
| Type          | Request          | Description   |Auth |  
|:------------- |:-----------------| -------------:|-------------:|
| GET           | /                | Entrypoint of the app ||
| POST          | /signup          |         User Signup||
| POST          | /login           |         User Login||
| GET           | /info            |     Get detail of medicine by  his id||
| GET           | /users           |          Get all users| * |
| GET           | /users/detail    |          Get current user detail| * |
| GET           | /users/{id}      |         Get user by id| * |
| GET           | /users/{id}      |         Get user by id| * |
| POST          | /medicines/new   |        Reg new medicine| * |
| GET           | /medicines       |        Get list medicines of company| * |
| DELETE        | /medicines/delete|        Get list medicines of company| * |
