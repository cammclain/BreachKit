from litestar import get, post, delete, put, Router, Controller, Request
from typing import Dict, List, Optional

# Simulated in-memory store for models (replace with actual logic later)
MODELS = {"red_llama": {"status": "loaded", "description": "A test model"}}

class OllamaController(Controller):
    path = "/ollama"

    @get("/")
    async def root(self) -> Dict[str, str]:
        """Root endpoint to check the status of the Ollama server."""
        return {"message": "Ollama Server is running"}

    @get("/models")
    async def list_models(self) -> Dict[str, List[str]]:
        """List all available models."""
        return {"models": list(MODELS.keys())}

    @get("/models/{model_name}")
    async def get_model(self, model_name: str) -> Optional[Dict[str, str]]:
        """Get details about a specific model."""
        model = MODELS.get(model_name)
        if not model:
            return {"error": f"Model '{model_name}' not found"}, 404
        return model

    @post("/models/{model_name}/load")
    async def load_model(self, model_name: str) -> Dict[str, str]:
        """Load a model into memory."""
        if model_name in MODELS:
            return {"message": f"Model '{model_name}' is already loaded"}
        MODELS[model_name] = {"status": "loaded", "description": f"Model '{model_name}' loaded"}
        return {"message": f"Model '{model_name}' loaded successfully"}

    @delete("/models/{model_name}/unload")
    async def unload_model(self, model_name: str) -> Dict[str, str]:
        """Unload a model from memory."""
        if model_name not in MODELS:
            return {"error": f"Model '{model_name}' not found"}, 404
        del MODELS[model_name]
        return {"message": f"Model '{model_name}' unloaded successfully"}

    @post("/models/{model_name}/run")
    async def run_model(self, model_name: str, request: Request) -> Dict[str, str]:
        """Run a model with given input."""
        model = MODELS.get(model_name)
        if not model:
            return {"error": f"Model '{model_name}' not found"}, 404

        data = await request.json()
        input_data = data.get("input", "")
        # Simulate running the model (replace with actual logic)
        result = f"Model '{model_name}' processed input: {input_data}"
        return {"result": result}
