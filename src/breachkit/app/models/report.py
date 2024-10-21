from pydantic import BaseModel
from typing import Optional
from surrealdb import SurrealDB

class Report(BaseModel):
    id: Optional[str] = None
    operation_id: str
    target_id: str
    content: str
    status: str

    @classmethod
    def create(cls, operation_id: str, target_id: str, content: str) -> Report:
        """Create a new report entry."""
        db = SurrealDB()
        result = db.insert("reports", {
            "operation_id": operation_id,
            "target_id": target_id,
            "content": content,
            "status": "in-progress"
        })
        return cls(**result)

    @classmethod
    def get(cls, report_id: str) -> Optional[Report]:
        db = SurrealDB()
        result = db.select("reports", report_id)
        return cls(**result) if result else None
