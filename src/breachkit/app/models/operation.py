from pydantic import BaseModel, Field
from typing import Optional, List
from surrealdb import SurrealDB

# import models
from .task import Task
from .scope import Scope

class Operation(BaseModel):
    id: Optional[str] = None
    name: str
    tasks: List[Task] = Field(default_factory=list)
    scope: Scope
    result: Optional[str] = None


    @classmethod
    def create(cls, name: str, target: str) -> Operation:
        db: SurrealDB = SurrealDB()
        result = db.insert("operations", {"name": name, "target": target})
        return cls(**result)

    def update(self, result: str) -> None:
        db: SurrealDB = SurrealDB()
        db.update("operations", self.id, {"result": result})
