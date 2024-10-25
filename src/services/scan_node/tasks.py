from __future__ import annotations

import logging

logger = logging.getLogger(__name__)
import redis
#from .scanners import bbot, custom_crawler

r = redis.Redis(host="localhost", port=6379)

'''def start_recon(target):
    # Trigger BBOT for initial infrastructure scan
    infrastructure = bbot.run_scan(target)
    r.publish('new_scan', infrastructure)

    # Follow up with a deeper OSINT scan if emails are found
    if infrastructure.get("emails"):
        custom_crawler.osint_scan(infrastructure["emails"])
'''