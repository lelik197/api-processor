### Backend for Mobile app service

### Run guide:
```
git clone https://github.com/lelik197/api-processor
```
#### Run with Docker

```
cd api-processor
docker compose up --build
```

#### Run without Docker
```
cd api-processor
cargo run
```

Service available on http://localhost:3000/api/calc

### Postman guide
Select POST on url: http://localhost:3000/api/calc
- Select headers:

``Key= Content-Type``

``Value= application/json``


- in Body select raw

demo JSON:

```
{
  "device_count": 15,
  "device_price": 52990.99,
  "servers_count": 2,
  "server_price": 200000,
  "operation": "calculate"
}
```