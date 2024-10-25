from __future__ import annotations
from litestar import Litestar
from app.routes.ollama_routes import OllamaController

app: Litestar = Litestar(
    route_handlers=[OllamaController],
    on_startup=[],
    on_shutdown=[],
    port=11433,
)
