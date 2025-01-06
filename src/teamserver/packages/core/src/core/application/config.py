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

    # Redis
    redis_url: str = os.getenv("REDIS_URL")

    