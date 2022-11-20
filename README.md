![Server](https://github.com/pprettysimpple/sd-raytracer/actions/workflows/server.yml/badge.svg)\

# sd-raytracer

### Запуск сервера

Возможно в системе нет протобуфов по умолчанию, нужно скачать их компилятор:

Ubuntu: `apt install protobuf-compiler`

Потом собираем и запускаем:

- `cd raytracer-server`
- `cargo build`
- `cargo run -- --address "127.0.0.1" --port 4242` (по умолчанию такие аргументы)
