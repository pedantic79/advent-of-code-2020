# Common module for Advent of Code

Use as a git subtree

## With remote

Setup:
```bash
git remote add -f common https://github.com/pedantic79/advent-of-code-common.git
git subtree add --prefix=src/common common main --squash
```

Update:
```bash
git subtree push --prefix=src/common common main
```

Pull:
```bash
git subtree pull --prefix=src/common common main --squash
```


## Without remote

Setup:
```bash
git subtree add --prefix=src/common https://github.com/pedantic79/advent-of-code-common.git main --squash
```

Update:
```bash
git subtree push --prefix=src/common https://github.com/pedantic79/advent-of-code-common.git main
```

Pull:
```bash
git subtree pull --prefix=src/common https://github.com/pedantic79/advent-of-code-common.git main --squash
```
