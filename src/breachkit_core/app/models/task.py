from __future__ import annotations
from surrealdb import SurrealDB
from pydantic import BaseModel
from typing import Optional

class Task(BaseModel):

    id: Optional[str] = None
    name: str
    description: Optional[str] = None

    @classmethod
    def create(cls, name: str, description: Optional[str] = None) -> Task:
        db: SurrealDB = SurrealDB()
        result = db.insert("tasks", {"name": name, "description": description})
        return cls(**result)




