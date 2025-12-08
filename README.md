### Backend for Mobile app service

### Run guide:
``cargo run``

Service available on http://localhost:3000/api/calc

##### Postman guide
Select POST on url: http://localhost:3000/api/calc
Select headers:

Key= Content-Type

Value= application/json

in Body select raw

demo JSON:

``"value_a": 10.5,
"value_b": 5.0,
"operation": "add"
}``