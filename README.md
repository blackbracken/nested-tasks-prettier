# nested-tasks-prettier

[WIP] A cli tool to pretty checkbox bullets which like markdown notation.

## How to use (C&P)

```shell
$ pbpaste | nested-tasks-prettier | pbcopy
```

## Examples

### `$ nested-tasks-prettier`

```plain
- [x] xxxxxx
- [>] yyyyyy
  - [-] aaaaa
  - [>] bbbbb
- [ ] zzzzzz
```

```plain
- ✅ xxxxxx
- 🚧 yyyyyy
  - 🛑 aaaaa
  - 🚧 bbbbb
- 📦 zzzzzz
```

## Roadmap

- ✅ decorate status emojis
- 🚧 hide and count nested items
- 📦 propose a status change for parent tasks if their child ones have been completed

<!--
- [x] decorate status emojis
- [>] hide and count nested items
- [ ] propose a status change for parent tasks if their child ones have been completed
-->
