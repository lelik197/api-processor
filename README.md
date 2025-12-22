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

Swagger available on http://localhost:3000/swagger-ui/

### Postman guide
Select POST on url: http://localhost:3000/api/calc
- Select headers:

``Key = Content-Type``

``Value = application/json``


- in Body select raw JSON

demo JSON:

```
{
  "device_count": 15,
  "device_price": 52990.99,
  "servers_count": 2,
  "server_price": 200000,
  "operation": "calculate",
  "developer_count": 3,
  "developer_salary": 123321,
  "qa_count": 1,
  "qa_salary": 70000

}
```


```
{
  "device_count": 15,
  "device_price": 52990.99,
  "servers_count": 2,
  "server_price": 200000,
  "operation": "calculate"
}
```