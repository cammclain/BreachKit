# **System Architecture**

This document provides a high-level overview of the architecture and how components interact within **BreachKit**.

---

## **Component Overview**

- **Litestar Framework:** Handles API requests and serves the dashboard.
- **SurrealDB:** Stores all structured data such as targets and scan results.
- **Redis:** Manages task queues and background processes.
- **Ollama:** Provides LLM-based insights for generating detailed reports.
- **SeaweedFS:** Stores large artifacts such as raw scan logs.

---

## **Component Interaction**

1. User triggers a scan through the dashboard (Litestar).
2. Redis queues the scan task and communicates with BBot.
3. BBot stores results in SurrealDB.
4. The user requests a report, and Ollama generates the analysis.
5. Reports and artifacts are stored in SeaweedFS.
