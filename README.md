
# **BreachKit**  

A **modern reconnaissance and vulnerability assessment platform** built for offensive security professionals. BreachKit empowers red team operations with a robust, scalable, and modular platform designed to streamline reconnaissance, scanning, and reporting efforts.

---

## **Table of Contents**  
- [Overview](#overview)  
- [Features](#features)  
- [Tech Stack](#tech-stack)  
- [Recon Tools](#recon-tools)  
- [Usage](#usage)  
- [License](#license)  
- [Acknowledgments](#acknowledgments)

---

## **Overview**  
BreachKit is a **web-based toolkit** designed to simplify reconnaissance and vulnerability detection in red team engagements. The platform leverages a mix of **powerful tools and services** to automate scanning, organize findings, and generate actionable reports.

With an **intuitive dashboard** for tracking operations, BreachKit allows users to generate **detailed LLM-powered reports** using Ollama’s self-hosted inference engine. Whether you're a novice or an experienced red teamer, BreachKit provides a **scalable and customizable platform** for offensive security intelligence.

---

## **Features**  
✨ **Fast and Modular:** Built with Docker, Litestar, and SurrealDB to ensure fast, portable deployments and smooth scaling.  
✨ **Distributed File Storage:** SeaweedFS stores large scan reports and logs efficiently.  
✨ **LLM-Powered Reports:** Uses Ollama’s LLM to provide detailed analysis and recommendations for detected vulnerabilities.  
✨ **Comprehensive Recon Tools:** Integrates Nmap, Nuclei, and BBot to cover multiple aspects of reconnaissance.  
✨ **Secure Access:** Deployed via **Tor hidden services** to ensure secure and anonymous usage.  

---

## **Tech Stack**  
The project is powered by the following technologies to ensure a seamless user experience:

### **Backend**  
- **[Litestar](https://litestar.dev/):** A fast ASGI web framework for building scalable applications.  
- **[SurrealDB](https://surrealdb.com/):** A distributed, schema-less database to manage and organize scan data.  
- **[Ollama](https://ollama.com/):** A self-hosted LLM API that powers BreachKit’s automated report generation.  
- **[SeaweedFS](https://seaweedfs.com/):** A distributed file system for handling large reports and data blobs.  
- **[Docker](https://docker.com/):** Containerization platform for portability and easy deployment.

### **Frontend**  
- **[Bulma](https://bulma.io/):** A modern CSS framework for responsive UI design without JavaScript.

---

## **Recon Tools**  
BreachKit integrates multiple industry-standard reconnaissance tools for thorough vulnerability detection:

- **[BBot](https://github.com/blacklanternsecurity/bbot):** A multi-modal reconnaissance tool that automates discovery and vulnerability analysis.  
- **[Nuclei](https://nuclei.projectdiscovery.io/):** A fast framework for vulnerability detection using customizable templates.  
- **[Nmap](https://nmap.org/):** A powerful network scanning tool to map open ports and services.

---

## **Usage**  
BreachKit’s workflow is designed to be intuitive and effective:

### **Performing a Recon Scan**  
1. **Log in to the Dashboard:** Access the **BreachKit dashboard** through the Tor hidden service.  
2. **Add a New Target:** Use the "Add Target" feature to specify the system or organization you want to scan.  
3. **Initiate a BBot Scan:** Start a scan using BBot directly from the dashboard.  
4. **Review Reports:** View the results in the "Reports" section, including LLM-generated analysis for each finding.

### **Generating Reports with Ollama**  
- **Automated Reports:** The platform uses Ollama to analyze scan results and generate detailed recommendations.  
- **Export Options:** Reports can be exported as **PDFs or Markdown** for sharing and further analysis.

---

## **License**  
This project is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for more information.

---

## **Acknowledgments**  
Special thanks to **Black Lantern Security** for BBot and to the **open-source community** for their contributions, which made this project possible.

