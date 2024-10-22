from __future__ import annotations

from pydantic import BaseModel, Field
from typing import Optional, List
from surrealdb import SurrealDB
from datetime import datetime


from .operator import Operator

class Alert(BaseModel):
    id: Optional[str] = None
    operation_id: str
    recieving_operators: List[Operator] = Field(default_factory=list)
    target_id: str
    vulnerability_id: str
    timestamp: datetime = Field(default_factory=datetime.now)


    @classmethod
    def create(cls, operation_id: str, target_id: str, vulnerability_id: str) -> Alert:
        db: SurrealDB = SurrealDB()
        result = db.insert("alerts", {"operation_id": operation_id, "target_id": target_id, "vulnerability_id": vulnerability_id})
        return cls(**result)
