# py-nuid

A Python bingings for https://github.com/casualjim/rs-nuid

Currently, on PyPi available build only for macOS, for another system you must have Rust compilator.

Example:
```python
from nuid import NUID

nuid = NUID()
print(nuid.next()) # mxFsAkDdbFyjesXY7vTn61

nuid.randomize_prefix()
print(nuid.next()) # xJfZpr3pGqGnesXY7vTn7u

```