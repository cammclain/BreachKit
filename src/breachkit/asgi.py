from __future__ import annotations
from litestar import Litestar
from litestar.template.config import TemplateConfig
from litestar.template.jinja import Jinja2TemplateEngine
from pathlib import Path


from breachkit.app.config.config import Config



def get_app() -> Litestar:

    config: Config = Config()

    app: Litestar = Litestar(
        template_config=TemplateConfig(
            directory=Path(__file__).parent / "app/templates",
            engine=Jinja2TemplateEngine(),
        ),
        route_handlers=[],
        on_startup=[],
        on_shutdown=[],
    )


    return app



