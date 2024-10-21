from __future__ import annotations

from pydantic import BaseModel
from bcrypt import hashpw, checkpw
from typing import Optional
from surrealdb import SurrealDB

class PGPKey(BaseModel):
    key: str
    fingerprint: str


class Operator(BaseModel):
    id: str
    name: str
    xmpp_address: str
    pgp_key: PGPKey
    role: str
    status: str
    password_hash: str

    def verify_password(self, password: str) -> bool:
        return checkpw(password.encode(), self.password_hash.encode())
    
    @classmethod
    def authenticate_operator(cls, xmpp_address: str, password: str) -> Optional[Operator]:
        operator = cls.get(xmpp_address)
        if operator and operator.verify_password(password):
            return operator
        return None

    @classmethod
    def get_operator(cls, xmpp_address: str) -> Optional[Operator]:
        db = SurrealDB()
        result = db.select("operators", xmpp_address)
        return cls(**result) if result else None

