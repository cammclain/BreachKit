from __future__ import annotations
import asyncio
from bbot.scanner import Scanner
from breachkit.app.config.config import Config
from typing import Any

class Scanner:
    def __init__(self, config: Config):
        self.config = config
        self.scanner = Scanner(config=config)


    async def scan(self, target: str) -> dict[str, Any]:
        return await self.scanner.scan(target)


    