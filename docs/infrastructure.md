# **Infrastructure Overview**

This document outlines the infrastructure powering **BreachKit** and the roles of various components within the system.

---

## **1. Backend Services**

1. **Litestar Framework**  
   - Manages API requests and serves the web dashboard.
   - Runs as an ASGI application inside the Docker container.

2. **SurrealDB**  
   - Stores scan results, targets, and vulnerability data.
   - Supports real-time query capabilities for the dashboard.

3. **Redis**  
   - Acts as the message broker for task orchestration.
   - Powers background tasks, such as scanning and report generation.

4. **Ollama (LLM Engine)**  
   - Provides automated analysis and reporting based on scan data.
   - Runs locally in a self-hosted environment via Docker.

5. **SeaweedFS**  
   - Handles blob storage for large scan logs, reports, and artifacts.

---

## **2. Frontend and Dashboard**

- **Bulma CSS**  
  - Styles the UI without the need for JavaScript.
- **Tor Hidden Service**  
  - Exposes the dashboard securely to the internet through Tor.

---

## **3. Deployment Architecture**

BreachKit is **containerized using Docker** and deployed as a **multi-service application**:

- **Backend Services:** Litestar, SurrealDB, Redis, Ollama
- **Frontend Service:** NGINX acting as a reverse proxy
- **Orchestration:** Docker Compose manages all the containers and networking.

---

## **4. Networking Setup**

- **Tor Hidden Services:**  
  - The dashboard and API are exposed via .onion addresses for secure and anonymous access.
  - NGINX acts as a reverse proxy to route requests internally.

---

## **5. Security Considerations**

- **Minimal logging:** Only essential logs are maintained to protect operational security.
- **Environment Isolation:** Each service runs in its own Docker container.
- **Tor Access:** The entire system is accessible exclusively through Tor hidden services to prevent external exposure.
