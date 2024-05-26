# Cloud user service

## environment variables

| Key | Type | Desc | Default |
|---|---|---|---|
|**DATABASE_URL**|_(string)_|the Postgres database URL|-|
|**MAX_DB_CONNECTION**|_(number)_|maximum connections in pool|4|
|**SECRET**|_(string)_|secret string for hashes, hmacs, jwt and etc.|-|
|**HTTP_HOST**|_(string)_|HTTP server host address|127.0.0.1|
|**HTTP_PORT**|_(number)_|HTTP server port number|8000|
