![Server](https://github.com/pprettysimpple/sd-raytracer/actions/workflows/server.yml/badge.svg)
![Client](https://github.com/pprettysimpple/sd-raytracer/actions/workflows/client.yml/badge.svg)

# sd-raytracer

### Запуск сервера

Возможно в системе нет протобуфов по умолчанию, нужно скачать их компилятор:

Ubuntu: `apt install protobuf-compiler`

Потом собираем и запускаем:

- `cd raytracer-server`
- `cargo build`
- `cargo run -- --address "127.0.0.1" --port 4242` (по умолчанию такие аргументы)

### Запуск клиента

_Вам все также потребуется компилятор протобуфов_

Запустить клиент вероятнее всего можно командой:
```sh
$ ./gradlew run
```
Клиент представляет из себя простенький REPL. Список команд:
- `set_fov ${double}` - выставить угол обзора
- `set_resolution ${int} ${int}` - выставить разрешение (ширина, высота)
- `set_origin ${double} ${double} ${double}` - выставить точку обзора (x, y, z)
- `set_view_direction ${double} ${double} ${double}` - выставить направление обзора (x, y, z)
- `set_filepath ${string}` - выставить путь до файла со сгенерированной картинкой (без расширения)
- `render` - зарендерить картинку
- `exit` - закончить интерактивную сессию
