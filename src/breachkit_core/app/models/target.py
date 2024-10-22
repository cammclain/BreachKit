from __future__ import annotations

from pydantic import BaseModel, Field
from typing import Optional, List
from surrealdb import SurrealDB

# import models
from .vulnerability import Vulnerability

class Target(BaseModel):
    id: Optional[str] = None
    name: str
    description: Optional[str] = None
    status: str
    vulnerabilities: List[Vulnerability] = Field(default_factory=list)

    @classmethod
    def create(cls, name: str, description: Optional[str] = None) -> Target:
        db: SurrealDB = SurrealDB()
        result = db.insert("targets", {"name": name, "description": description, "status": "active"})
        return cls(**result)

    @classmethod
    def get(cls, target_id: str) -> Optional[Target]:
        db: SurrealDB = SurrealDB()
        result = db.select("targets", target_id)
        return cls(**result) if result else None
