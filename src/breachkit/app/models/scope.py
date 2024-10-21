from __future__ import annotations

from pydantic import BaseModel, Field
from typing import List, Optional
from surrealdb import SurrealDB

from .target import Target
class Scope(BaseModel):
    id: str
    operation_id: str
    targets: List[Target] = Field(..., description="List of target IDs in scope")
    
    def save(self) -> None:
        """Save the scope to SurrealDB."""
        db: SurrealDB = SurrealDB()
        db.insert("scopes", self.dict())

    @classmethod
    def create(cls, operation_id: str, target_ids: List[str], description: Optional[str] = None) -> Scope:
        db: SurrealDB = SurrealDB()
        result = db.insert("scopes", {"operation_id": operation_id, "targets": target_ids, "description": description})
        return cls(**result)


