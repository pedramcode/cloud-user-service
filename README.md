# Cloud user service

## Todo list

- [x] design and implement SOLID project structure
- [x] user models
- [x] user repository
- [x] user service
- [x] otp models
- [x] otp repository
- [x] otp service
- [x] JWT
- [x] setup Rocket web server
- [x] setup structure for routing
- [x] setup root controllers
- [x] setup user controllers
- [x] setup JWT controllers
    - [x] refresh
    - [x] access
    - [x] verify
- [ ] RBAC

## environment variables

| Key | Type | Desc | Default |
|---|---|---|---|
|**DATABASE_URL**|_(string)_|the Postgres database URL|-|
|**MAX_DB_CONNECTION**|_(number)_|maximum connections in pool|4|
|**SECRET**|_(string)_|secret string for hashes, hmacs, jwt and etc.|-|
|**HTTP_HOST**|_(string)_|HTTP server host address|127.0.0.1|
|**HTTP_PORT**|_(number)_|HTTP server port number|8000|
