# Calculator

A recursive descent parser for mathematic expressions based on the
grammar in [Jamis Buck][jamis]'s [parser challenge][challenge].

[jamis]: https://github.com/jamis
[challenge]: http://weblog.jamisbuck.org/2016/9/17/weekly-programming-challenge-8.html

## Usage

```
$ calc 1 + 2

$ calc --dot 1 + 2 | dot -Tsvg > tree.svg
```

## Development

```
$ cargo test
$ cargo build
```

## License

Calculator is released under the MIT license. Check the LICENSE file for details.
