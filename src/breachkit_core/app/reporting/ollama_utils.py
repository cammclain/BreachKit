from __future__ import annotations
# these files are used for generating markdown reports using ollama

from ollama import OllamaClient

from surrealdb import Surreal
from ollama import Ollama

# Connect to SurrealDB
db = Surreal("ws://localhost:8000/rpc")
db.signin({"user": "user", "pass": "password"})
db.use("breachkit", "recon_data")

# Load fabric pattern
FABRIC_PATTERN = """
# Report: Organizational Reconnaissance

## Executive Summary:
- Organization: {organization_name}
- Targeted Systems: {targeted_assets}
- Number of Employees Identified: {employee_count}
- Vulnerabilities Discovered: {vulnerabilities_count}

## Detailed Findings:
1. **Infrastructure**: 
    - IPs and Domains: {domains}
    - SSL/TLS Issues: {ssl_issues}
    - Open Ports: {open_ports}

2. **Employees and Credentials**:
    - Employee Emails Identified: {emails_found}
    - Breached Credentials: {breached_credentials}
    - Potential Social Profiles: {social_profiles}

3. **Vulnerabilities**:
    - Critical Vulnerabilities: {critical_vulns}
    - CVEs Identified: {cves}

## Risk Analysis:
Primary risks identified:
1. {risk_1}
2. {risk_2}

## Recommended Actions:
- Short-Term: {short_term_recommendations}
- Long-Term: {long_term_recommendations}

## Next Steps:
1. {next_scan_targets}
2. {potential_weak_points}
"""

# Query data from SurrealDB
def get_target_data(target_id):
    query = f"SELECT * FROM targets WHERE id = '{target_id}'"
    result = db.query(query)
    return result[0] if result else {}

# Generate the report using Ollama and fabric pattern
def generate_report(target_id):
    data = get_target_data(target_id)
    report = FABRIC_PATTERN.format(
        organization_name=data.get("name", "Unknown"),
        targeted_assets=data.get("assets", []),
        employee_count=len(data.get("employees", [])),
        vulnerabilities_count=len(data.get("vulnerabilities", [])),
        domains=data.get("domains", []),
        ssl_issues=data.get("ssl_issues", []),
        open_ports=data.get("open_ports", []),
        emails_found=data.get("emails", []),
        breached_credentials=data.get("breached", []),
        social_profiles=data.get("social_profiles", []),
        critical_vulns=data.get("critical_vulnerabilities", []),
        cves=data.get("cves", []),
        risk_1="Unauthorized access to sensitive systems",
        risk_2="Potential exposure of customer data",
        short_term_recommendations="Disable vulnerable services immediately.",
        long_term_recommendations="Implement continuous vulnerability management.",
        next_scan_targets="Employee email exposure.",
        potential_weak_points="SSL certificate misconfigurations."
    )

    ollama = Ollama(model="reporter")
    summary = ollama.generate(report)
    return summary

# Example usage
if __name__ == "__main__":
    print(generate_report("example_id"))
