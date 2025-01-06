from robyn import Robyn
from .config import Config
from .database import Database
from logging import Logger
from redis import Redis

def create_operator_app():
    operator_app = Robyn(__file__)

    # load the config
    config = Config()
    
    # initialize the logger
    logger = Logger(config.log_level)
    # initialize the database
    db = Database(config.db_url)

    # initialize the redis client
    redis_client = Redis(config.redis_url)



    return operator_app

