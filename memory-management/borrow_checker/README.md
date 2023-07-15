## Borrow Checker
A variable have three permissions on their data
- **Read(R)**: data can be copied to another location.
- **Write(W)**: data can be mutated in-place.
- **Own(O)**: data can be moved or dropped.
- **Flow(F)**:  expected whenever an expression uses an input reference

## Key Points:
- **References** are non-owning pointers, because they do not own the data they point to.
- By default, a variable has read/own permissions (RO) on its data.
- With ` let mut ` a variable has write permissions (W) on its data.
- References can temporarily remove these permissions.
- Mutable References provide unique and non-owning access to data.
-  A reference changes permissions while it is "in use". The phrase "in use" is describing a reference's lifetime.