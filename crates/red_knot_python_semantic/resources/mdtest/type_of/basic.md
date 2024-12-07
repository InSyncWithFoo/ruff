# type special form

## Class literal

```py
class A: ...

def f() -> type[A]:
    return A

reveal_type(f())  # revealed: type[A]
```

## Nested class literal

```py
class A:
    class B: ...

def f() -> type[A.B]:
    return A.B

reveal_type(f())  # revealed: type[B]
```

## Deeply nested class literal

```py
class A:
    class B:
        class C: ...

def f() -> type[A.B.C]:
    return A.B.C

reveal_type(f())  # revealed: type[C]
```

## Class literal from another module

```py
from a import A

def f() -> type[A]:
    return A

reveal_type(f())  # revealed: type[A]
```

```py path=a.py
class A: ...
```

## Qualified class literal from another module

```py
import a

def f() -> type[a.B]:
    return a.B

reveal_type(f())  # revealed: type[B]
```

```py path=a.py
class B: ...
```

## Deeply qualified class literal from another module

```py path=a/test.py
import a.b

# TODO: no diagnostic
# error: [unresolved-attribute]
def f() -> type[a.b.C]:
    # TODO: no diagnostic
    # error: [unresolved-attribute]
    return a.b.C

reveal_type(f())  # revealed: @Todo(unsupported type[X] special form)
```

```py path=a/__init__.py
```

```py path=a/b.py
class C: ...
```

## Union of classes

```py
class BasicUser: ...
class ProUser: ...

class A:
    class B:
        class C: ...

def get_user() -> type[BasicUser | ProUser | A.B.C]:
    return BasicUser

# revealed: type[BasicUser] | type[ProUser] | type[C]
reveal_type(get_user())
```

## `Any`

### Graduality

```py
from typing import Any

def type_any() -> type[Any]: ...

reveal_type(type_any())  # revealed: type[Any]
reveal_type(type_any().foo)  # revealed: Any
```

### Assignability

```py
from typing import Any

def type_any() -> type[Any]: ...
def type_int() -> type[int]: ...

a: type[Any] = int
b: type[Any] = type_int()
c: type[int] = type_any()

d: type[Any] = type
e: type = type_any()
```

## Member resolving

```py
from typing import Any

def type_any() -> type[Any]: ...

# revealed: tuple[Literal[type], Literal[object]]
reveal_type(type_any().__mro__)
# revealed: str
reveal_type(type_any().__name__)
```

## Illegal parameters

```py
class A: ...
class B: ...

# error: [invalid-type-form]
def get_user() -> type[A, B]:
    return A
```
