from typing import Union

front: None | A | B
middle: A | None | B
back: A | B | None

front_old: Union[None, A, B]
middle_old: Union[A, None, B]
back_old: Union[A, B, None]

mixed_front: Union[None, A | B, C]
mixed_middle: Union[A | B, None, C]
mixed_back: Union[A | B, C, None]

mixed_2_front: None | Union[A, B] | C
mixed_2_middle: Union[A, B] | None | C
mixed_2_back: Union[A, B] | C | None

complex_1_front: None | tuple[None, int] | Callable[[], None]
complex_1_middle: tuple[None, int] | None | Callable[[], None]
complex_1_back: tuple[None, int] | Callable[[], None] | None

complex_1_front: Union[None, tuple[None, None], Callable[[], None]]
complex_1_middle: Union[tuple[None, None], None, Callable[[], None]]
complex_1_back: Union[tuple[None, None], Callable[[], None], None]

# None | int | str | bytes | None | None | dict[None, list[None]]
nested: Union[Union[None, int], str | Union[bytes, None] | Union[Union[None]]] | dict[None, list[None]]

empty: None | Union[()]
