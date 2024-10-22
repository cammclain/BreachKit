from __future__ import annotations
from pydantic import BaseModel, Field, validator
from bcrypt import hashpw, gensalt, checkpw
from typing import Optional
from surrealdb import SurrealDB

class PGPKey(BaseModel):
    key: str
    fingerprint: str

class Operator(BaseModel):
    id: Optional[str] = None
    name: str = Field(..., max_length=100)
    xmpp_address: str = Field(..., regex=r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
    pgp_key: PGPKey
    role: str
    status: str
    password_hash: Optional[str] = None

    @validator("password_hash", pre=True, always=True)
    def hash_password(cls, value: Optional[str]) -> Optional[str]:
        """Hashes the password if provided."""
        return hashpw(value.encode(), gensalt()).decode() if value else None

    def verify_password(self, password: str) -> bool:
        """Check if the provided password matches the stored hash."""
        return checkpw(password.encode(), self.password_hash.encode())

    @classmethod
    def authenticate_operator(cls, xmpp_address: str, password: str) -> Optional[Operator]:
        operator = cls.get_operator(xmpp_address)
        if operator and operator.verify_password(password):
            return operator
        return None

    @classmethod
    def get_operator(cls, xmpp_address: str) -> Optional[Operator]:
        db = SurrealDB()
        result = db.select("operators", xmpp_address)
        return cls(**result) if result else None
