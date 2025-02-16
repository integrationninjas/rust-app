# rust-app

# User CRUD API

## Overview
This is a simple CRUD API for managing users using Actix Web and MongoDB.

## Prerequisites
- Rust installed
- MongoDB running
- Docker (optional, for containerization)

## Running the API
### Locally
Create a `.env` file with:
```
MONGO_URI=mongodb://localhost:27017
DATABASE_NAME=my_database
```
Then, run:
```sh
cargo run
```

### Using Docker
Build and run the container:
```sh
docker build -t rust-app .
docker run -p 5050:3030 rust-app
```

## API Endpoints

### 1. Get All Users
**GET** `/users`
```sh
curl --location 'http://localhost:5050/users'
```

### 2. Create a New User
**POST** `/users`
```sh
curl --location 'http://localhost:5050/users' \
--header 'Content-Type: application/json' \
--data-raw '{
    "name": "John Doe",
    "email": "john@example.com",
    "age": 30
}'
```

### 3. Update a User
**PUT** `/users/{user-id}`
```sh
curl --location --request PUT 'http://localhost:5050/users/<user-id>' \
--header 'Content-Type: application/json' \
--data-raw '{
    "name": "John Doe",
    "email": "john.doe@example.com",
    "age": 37
}'
```

### 4. Delete a User
**DELETE** `/users/{user-id}`
```sh
curl --location --request DELETE 'http://localhost:5050/users/<user-id>'
```