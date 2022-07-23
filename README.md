# nested-tasks-prettier

[WIP] / A cli tool to pretty checkbox bullets which like markdown notation.

## How to use (C&P)

```shell
$ pbpaste | ./nested-tasks-prettier | pbcopy
```

## Example

### Input

```plain
- [x] xxxxxx
- [>] yyyyyy
  - [-] aaaaa
  - [>] bbbbb
- [ ] zzzzzz
```

### Output

```plain
- ✅ xxxxxx
- 🚧 yyyyyy
  - 🛑 aaaaa
  - 🚧 bbbbb
- 📦 zzzzzz
```

## Roadmap

- ✅ decorate status emojis
- 🚧 propose a status change for parent tasks if their child ones have been completed
- 🚧 hide nested items

<!--
- [x] decorate status emojis
- [>] propose a status change for parent tasks if their child ones have been completed
- [>] hide nested items
-->
