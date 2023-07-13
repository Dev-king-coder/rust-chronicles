## Borrow Checker
A variable have three permissions on their data
- **Read(R)**: data can be copied to another location.
- **Write(W)**: data can be mutated in-place.
- **Own(O)**: data can be moved or dropped.

## Key Points:
- **References** are non-owning pointers, because they do not own the data they point to.
- By default, a variable has read/own permissions (RO) on its data.
- With ` let mut ` a variable has write permissions (W) on its data.
- References can temporarily remove these permissions