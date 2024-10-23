# Система мониторинга и оповещения об изменениях в базе данных

Cистема мониторинга, которая отслеживает изменения в базе данных PostgreSQL и отправляет оповещения в Kafka.


## Запуск приложения 

### 1. Запуск Kafka и Postgres

```bash
  docker-compose up -d
```
### 2. Накатывание миграций на Postgres

```bash
  sqlx migrate run --database-url <POSTGRES_ADDRESS>
```


### 3. Запуск сервера 

```bash
  cargo run -- -a <ADDRESS> -p <CONFIG_PATH>
```
-  `-a <ADDRESS>`: адрес, на котором будет запущен сервер

-  `-p <CONFIG_PATH>`: путь до файла конфигурации `(*.toml)` , содержащего настройки для подключения к PostgreSQL и Kafka.
###### Пример файла конфигурации   
```toml
[app_config]
    [postgres]
        host="localhost"
        port=5432
        login="wb_tech"
        password="wb_tech"
        db="L3.4"
        max_connections=100
        idle_timeout_sec=1200
        max_lifetime_sec=2
        acquire_time_sec=1
    [kafka]
        topic_name="L3.4"
        addr="localhost:9092"
```
## API приложения 
#### Cоздание нового пользователя 
```http
  POST /users
  Content-Type: application/json
{
    "name":"asdasd",
    "email":"asdss@asdasds"
}
```
#### Обновление информации о пользователе
```http
  PUT /users/:id
  Content-Type: application/json
{
    "name":"asdasd", //Опционально
    "email":"asdss@asdasds" //Опционально
}
```
#### Удаление пользователя
```http
  POST /users/:id
```
## API для продукта
#### Cоздание нового продукта
```http
  POST /products
  Content-Type: application/json
{
    "name":"cake4",
    "price":{
        "rubles":1,
        "kopecs":99
    }
}
```
#### Обновление информации о продукте
```http
  PUT /products/:id
  Content-Type: application/json
{
    "name":"cake4" //Опционально
    "price": { 
        "rubles":2 //Опционально
        "kopecs":0 //Опционально
    } //Опционально
}
```
#### Удаление продукта
```http
  DELETE /products/:id
```