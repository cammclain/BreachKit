from robyn import Robyn
from .application import create_operator_app

if __name__ == "__main__":
    # create a configured "Session" class
    operator_app: Robyn = create_operator_app()
    operator_app.start(host="0.0.0.0", port=8080)
