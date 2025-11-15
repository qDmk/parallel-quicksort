# Практическое заданиe №1: quicksort

## Реализация

Реализация алгоритма quicksort находится в файле [src/lib.rs](src/lib.rs).
Код в файле [src/main.rs](src/main.rs) использовался для проверки качествa выбора pivot'а.

## Бенчмарки

Бенчмари находятся в [tests/test.rs](tests/test.rs).
Запустить через cargo ([установить тут](https://rustup.rs/)): `cargo bench`

Либо в docker:
```sh
docker build -t qdmk4-quicksort .
docker run qdmk4-quicksort
```

Бенчмарк производится при помощи крейта [`criterion`](https://docs.rs/criterion/latest/criterion/).
Усреднение по 10 запускам (criterion не позволяет установить меньше, он очень старается делать все возможное для статистической значимости).
Один поток и четыре потока запускаются на том же наборе входных данных.

Пример запуска:
```
  Running benches/benchmark.rs (target/release/deps/benchmark-3bb869e00f10b689)
Gnuplot not found, using plotters backend
Benchmarking quicksort x4
Benchmarking quicksort x4: Warming up for 3.0000 s

Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 39.3s.
Benchmarking quicksort x4: Collecting 10 samples in estimated 39.335 s (10 iterations)
Benchmarking quicksort x4: Analyzing
quicksort x4            time:   [3.6332 s 3.6648 s 3.7047 s]
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild
Benchmarking quicksort x1
Benchmarking quicksort x1: Warming up for 3.0000 s


Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 119.8s.
Benchmarking quicksort x1: Collecting 10 samples in estimated 119.77 s (10 iterations)
Benchmarking quicksort x1: Analyzing
quicksort x1            time:   [11.715 s 11.729 s 11.745 s]
Found 2 outliers among 10 measurements (20.00%)
  1 (10.00%) high mild
  1 (10.00%) high severe
```

Время в квадратных скобках: нижняя граница доверительного интервала, непосредственно оценка одной итерации, верхняя граница.

## Тесты

Тесты можно посмотреть в [tests/test.rs](tests/test.rs).
Запустить отедльно тесты: `cargo test` (дебаг сборка) и `cargo tests --release` (релизная сборка).
