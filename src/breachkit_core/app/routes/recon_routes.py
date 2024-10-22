from litestar import Router, get, post
from breachkit_core.app.models.operation import Operation
from breachkit_core.app.breaching.bbot_utils import run_bbot_scan

routes = Router(path="/recon")

@post("/start")
async def start_scan(target: str) -> dict:
    """
    Start a BBot scan on the provided target.
    """
    operation = Operation.create(name=f"Scan {target}", target=target)
    scan_result = await run_bbot_scan(target)
    operation.update(result=scan_result)

    return {"status": "Scan started", "operation_id": operation.id}

@get("/{operation_id}")
async def get_scan_result(operation_id: str) -> dict:
    """
    Retrieve the results of a completed scan.
    """
    operation = Operation.get(operation_id)
    if not operation:
        return {"error": "Operation not found"}, 404

    return {"operation": operation.to_dict()}
