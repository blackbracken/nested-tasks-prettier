# nested-tasks-prettier

[WIP] / A cli tool to pretty checkbox bullets which like markdown notation.

## how to use (C&P)

```shell
$ pbpaste | ./nested-tasks-prettier | pbcopy
```

## example

### input

```plain
- [x] xxxxxx
- [>] yyyyyy
  - [-] aaaaa
  - [>] bbbbb
- [ ] zzzzzz
```

### output

```plain
- ✅ xxxxxx
- 🚧 yyyyyy
  - 🛑 aaaaa
  - 🚧 bbbbb
- 📦 zzzzzz
```

## roadmap

- ✅ status emoji
- 📦 hide items

<!--
- [x] decorate status emoji
- [ ] hide nested items
-->
