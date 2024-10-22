
from __future__ import annotations
from litestar import Litestar
from litestar.template.config import TemplateConfig
from litestar.template.jinja import Jinja2TemplateEngine
from pathlib import Path
from breachkit_core.app.routes import admin_routes, recon_routes, reporting_routes
from breachkit_core.app.config.config import Config

def get_app() -> Litestar:
    """Initialize and configure the Litestar application."""
    config: Config = Config()

    app = Litestar(
        template_config=TemplateConfig(
            directory=Path(__file__).parent / "app/templates",
            engine=Jinja2TemplateEngine(),
        ),
        route_handlers=[
            admin_routes.routes,
            recon_routes.routes,
            reporting_routes.routes,
        ],
        on_startup=[config.initialize_database],
        on_shutdown=[config.close_database_connection],
    )

    return app

app = get_app()
