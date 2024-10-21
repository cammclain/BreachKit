from __future__ import annotations
from litestar import Litestar


from breachkit.app.config.config import Config



def get_app() -> Litestar:

    config: Config = Config()

    app: Litestar = Litestar(
        route_handlers=[],
        on_startup=[],
        on_shutdown=[],
    )


    return app



