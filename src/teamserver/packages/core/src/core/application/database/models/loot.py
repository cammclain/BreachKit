from ..db import Base
from sqlalchemy import Column, Integer, String, DateTime, Enum
from datetime import datetime



class LootType(Enum):
    CREDENTIAL: str = "credentials"
    PASSWORD_HASH: str = "password_hash"
    HASH: str = "hash"
    FILE: str = "file"
    FOLDER: str = "folder"
    ARCHIVE: str = "archive"
    URL: str = "url"
    DOMAIN: str = "domain"
    IP: str = "ip"

# Create the base loot model
class Loot(Base):
    __tablename__ = "loot"
    id = Column(Integer, primary_key=True, index=True)
    name = Column(String, unique=True, index=True)
    description = Column(String)
    created_at = Column(DateTime, default=datetime.now)
    updated_at = Column(DateTime, default=datetime.now, onupdate=datetime.now)
    type = Column(String, default=LootType.FILE)



class Credential(Base):
    __tablename__ = "credentials"
    id = Column(Integer, primary_key=True, index=True)
    username = Column(String, unique=True, index=True)
    password = Column(String)
    notes = Column(String)
    created_at = Column(DateTime, default=datetime.now)
    hash_type = Column(String)
    updated_at = Column(DateTime, default=datetime.now, onupdate=datetime.now)

