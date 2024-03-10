# Architecture of a Rust Rest API with JWT Authentication

## TECH STACK
- Rust
- Axum
- Sqlx
- Posgresql
- Redis
- Minio/S3


## START PROJECT
### create environment variables file
rename **.env.example** file  into **.env**

### run docker compose
```console
sudo docker-compose up -d
```

### run migrations
```console
sqlx migrate run --database-url postgres://rustblog:rustblog@localhost:8100/rustblog
```

### run server
```console
cargo run
```

Visit the [Swagger docs](http://localhost:8000/api/docs/#/)
