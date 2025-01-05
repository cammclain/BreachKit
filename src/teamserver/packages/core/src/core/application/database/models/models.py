from ..db import Base

from sqlalchemy import Boolean, Column, Integer, String
class User(Base):
    __tablename__ = "users"

    id = Column(Integer, primary_key=True, index=True)
    username = Column(String, unique=True, index=True)
    hashed_password = Column(String)
    is_active = Column(Boolean, default=True)
    is_superuser = Column(Boolean, default=False)

    def __repr__(self):
        return f"User(id={self.id}, username={self.username}, is_active={self.is_active}, is_superuser={self.is_superuser})"
