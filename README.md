# M3U playlist generator

## Установка

Перед установкой требуется создать базу данных MySQL.

```shell script
# Скопировать файл ".env.dist" в ".env".
cp .env.dist .env
```

В файле `.env` необходимо отредактировать значение переменной `DATABASE_URL` в соответствии с шаблоном `mysql://[user[:password]@]host/database_name`.

```
# Установить пакет "libmysqlclient-dev".
sudo apt-get install libmysqlclient-dev

# Установить Diesel CLI.
cargo install diesel_cli --no-default-features --features mysql

# Запустить существующие миграции.
diesel setup
```

## Использование

```shell script
# Скомпилировать и запустить.
cargo run
```

После запуска будет создан файл `playlist.m3u8`, содержащий плейлист.
