# Мини-социальная сеть для обмена сообщениями

## Запуск
### 1. Запуск Postgres
```bash
  docker-compose up -d
```
### 2. Запуск сервера         
```bash
  cargo run -- -p <PATH>
```
-  `-p <PATH>`: путь до файла конфигурации (*.toml) , содержащего настройки запуска сервера и для подключения к PostgreSQL
###### Пример файла конфигурации
```toml
[server_config]
    host="127.0.0.1"
    port=8080
    secret="wb_tech"
    exp_time_min=20
[postgres_config]
    host="localhost"
    port=5432
    login="wb_tech"
    password="wb_tech"
    db="social_network"
[migration_config]
    path="./migrations/v1"
```       
## API  
### Cоздание нового пользователя 
```json
POST /register
Content-Type: application/json
{
    "login":"example",
    "hashed_password":"example"
}
```
###### Ответ на успешную регистрацию
```json 
HTTP/1.1 200 Ok
Content-Type: application/json
Authorization: Bearer <token>
{
   user_id
}
```
##
#### Авторизация пользователя
```json
POST /login
Content-Type: application/json

{
    "user_uid":"3e9d068a-2057-402c-ad6a-d4405af78dc4",
    "login":"example",
    "hashed_password":"example"
}
```
###### Ответ на успешный логин
```json 
HTTP/1.1 200 Ok
Content-Type: application/json
Authorization: Bearer <token>
```
##
#### Создание нового сообщения 
```json
  POST /posts
  Content-Type: application/json
  Authorization: Bearer <token>
{
    "msg":"message"
}
```
#### Получение сообщения
```json
  GET /posts/{post_id}
  Content-Type: application/json
  Authorization: Bearer <token>
```

##
#### Удаление сообщения
```json
  DELETE /posts/{post_id}
  Content-Type: application/json
  Authorization: Bearer <token>
```
##
#### Лайк сообщения 
```json
  POST /posts/{post_id}/likes
  Content-Type: application/json
  Authorization: Bearer <token>
```