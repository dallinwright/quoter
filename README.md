# quoter
Quote Service Demo


```bash
curl --location 'http://localhost:8080/quote' \
--header 'X-User-Id: Albert Camus'
```


2025-10-25T17:47:21.980610Z  INFO get_quote_by_id_route{id=38bf0c59-c53d-4942-b8dc-92bb80092999 headers={"x-user-id": "Albert Camus", "user-agent": "PostmanRuntime/7.49.0", "accept": "*/*", "postman-token": "1056dbc8-84d4-4f74-9f18-5a676549f59a", "host": "localhost:8080", "accept-encoding": "gzip, deflate, br", "connection": "keep-alive"}}:get_quote_by_id{author="Albert Camus" id=38bf0c59-c53d-4942-b8dc-92bb80092999}: tiberius::tds::stream::token: 187: Packet size change from '4096' to '4096'
