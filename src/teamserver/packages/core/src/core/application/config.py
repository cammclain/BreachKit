import os

from pydantic import BaseModel

from dotenv import load_dotenv

load_dotenv()


class Config(BaseModel):
    # Database
    db_url: str = os.getenv("DB_URL")
    db_pool_size: int = 10
    db_pool_timeout: int = 30
    db_pool_max_lifetime: int = 300
    db_pool_max_idle: int = 10
    
    # JWT
    jwt_secret: str = os.getenv("JWT_SECRET")
    jwt_algorithm: str = os.getenv("JWT_ALGORITHM")
    jwt_access_token_expires: int = 3600
    jwt_refresh_token_expires: int = 86400

    # Redis
    redis_url: str = os.getenv("REDIS_URL")

    