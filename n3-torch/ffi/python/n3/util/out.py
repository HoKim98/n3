from typing import Dict


class Out:
    def __init__(self, id: int, name: str) -> None:
        self.id = id
        self.name = name

    def __eq__(self, other: 'Out') -> bool:
        return isinstance(other, type(self)) and (self.id, self.name) == (other.id, other.name)

    def __hash__(self) -> int:
        return hash((self.id, self.name))

    def __repr__(self) -> str:
        id = self.id if self.id is not None else ''
        name = self.name or ''
        return f'{name}${id}'


Outs = Dict[str, Out]
