


from dataclasses import dataclass
from typing import Any, Self


arena = []
last_id = 0

@dataclass
class Node:
    value : Any
    next : Self | None = None
    last_id: int | None = None
    

Node(45)