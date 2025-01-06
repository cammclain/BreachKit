from ..db import Base
from enum import Enum
from sqlalchemy import Boolean, Column, Integer, String, DateTime
from datetime import datetime


class UserRole(Enum):
    ADMIN: str = "admin"
    USER: str = "user"
    AGENT: str = "agent"

class UserStatus(Enum):
    ACTIVE: str = "active"
    INACTIVE: str = "inactive"
    PENDING: str = "pending"
    BANNED: str = "banned"

class Team(Base):
    __tablename__ = "teams"
    id = Column(Integer, primary_key=True, index=True)
    name = Column(String, unique=True, index=True)
    description = Column(String)
    created_at = Column(DateTime, default=datetime.now)
    updated_at = Column(DateTime, default=datetime.now, onupdate=datetime.now)


class User(Base):
    __tablename__ = "users"

    id = Column(Integer, primary_key=True, index=True)
    username = Column(String, unique=True, index=True)
    hashed_password = Column(String)
    is_active = Column(Boolean, default=True)
    is_superuser = Column(Boolean, default=False)

    def __repr__(self):
        return f"User(id={self.id}, username={self.username}, is_active={self.is_active}, is_superuser={self.is_superuser})"

