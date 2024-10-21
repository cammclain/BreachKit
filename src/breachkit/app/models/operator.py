from __future__ import annotations
from pydantic import BaseModel, validator, Field
from typing import Optional
from bcrypt import hashpw, checkpw, gensalt
from surrealdb import SurrealDB

class PGPKey(BaseModel):
    key: str = Field(..., description="The PGP public key")
    fingerprint: str = Field(..., description="The PGP fingerprint")

class Operator(BaseModel):
    id: str
    name: str
    xmpp_address: str
    pgp_key: PGPKey
    role: str 
    status: str
    password_hash: str

    @validator('password_hash', pre=True, always=True)
    def hash_password(cls, v: str) -> str:
        """Hash the password if not already hashed."""
        return hashpw(v.encode(), gensalt()).decode() if not v.startswith("$2b$") else v

    def verify_password(self, password: str) -> bool:
        """Verify the operator's password."""
        return checkpw(password.encode(), self.password_hash.encode())

    @classmethod
    def authenticate_operator(cls, xmpp_address: str, password: str) -> Optional[Operator]:
        """Authenticate an operator by XMPP address and password."""
        operator = cls.get_operator(xmpp_address)
        if operator and operator.verify_password(password):
            return operator
        return None

    @classmethod
    def get_operator(cls, xmpp_address: str) -> Optional[Operator]:
        """Retrieve an operator from SurrealDB."""
        db = SurrealDB()
        result = db.select("operators", xmpp_address)
        return cls(**result) if result else None

    def save(self) -> None:
        """Save the operator to SurrealDB."""
        db = SurrealDB()
        db.insert("operators", self.dict())
