# Enrichment Flows for Active Directory Data

This document elaborates on the enrichment flows designed to extract and analyze data from Active Directory (AD) environments. It highlights methodologies, tools, and tactics for both offensive and defensive use cases, serving as a comprehensive reference for cybersecurity professionals.

---

## Purpose

Active Directory is a critical component in many enterprise IT environments. Understanding its structure and potential vulnerabilities is key for:

1. **Incident Response**: Quickly identifying compromises.
2. **Threat Hunting**: Proactively searching for malicious activity.
3. **Red Team Operations**: Simulating adversary tactics to test defenses.
4. **Blue Team Operations**: Strengthening security measures based on simulated attack findings.

---

## Components of Enrichment Flows

### 1. **Data Sources**

- **Event Logs**: Security, application, and system logs.
- **Domain Metadata**: Extracted via tools like PowerView or BloodHound.
- **Network Traffic**: Analyzed for unusual patterns or anomalies.
- **Configuration Files**: Group Policy Objects (GPO), SYSVOL content, etc.
- **Stored Credentials**: LSA secrets, SAM, or registry artifacts.

### 2. **Collection Techniques**

- **Credential Dumping**: Using Mimikatz or Invoke-Mimikatz.
- **SPN Scanning**: Enumerating service accounts via Kerberos tickets.
- **BloodHound Queries**: Mapping relationships and paths to Domain Admin.
- **LDAP Queries**: Enumerating objects and permissions.

### 3. **Enrichment Methods**

- **Correlating Relationships**: Using BloodHound to identify trust paths.
- **Baseline Comparison**: Detecting deviations from normal behavior.
- **Cross-referencing Credentials**: Matching credentials across multiple domains or systems.
- **Metadata Analysis**: Parsing configurations for misconfigurations (e.g., excessive permissions).

---

## Tools and Utilities

### Offensive Tools
- **BloodHound**: Visualizing AD attack paths.
- **SharpHound**: Collecting AD data for BloodHound.
- **Mimikatz**: Credential extraction.
- **PowerView**: Enumerating AD.

### Defensive Tools
- **Sysmon**: Capturing detailed logs.
- **Splunk**: Aggregating and analyzing data.
- **Azure Sentinel**: Cloud-based threat detection.
- **PingCastle**: Identifying AD vulnerabilities.

---

## Example Enrichment Flows

### 1. **Credential Misuse Detection**
   **Objective**: Identify misuse of credentials within the domain.
   - **Step 1**: Extract credentials using tools like Mimikatz.
   - **Step 2**: Cross-reference credentials with paths in BloodHound.
   - **Step 3**: Identify instances of privilege misuse or misconfiguration.

### 2. **SPN Discovery for Lateral Movement**
   **Objective**: Enumerate service accounts to assess lateral movement opportunities.
   - **Step 1**: Run SPN scans using PowerView or Kerbrute.
   - **Step 2**: Verify service accounts against baseline configurations.
   - **Step 3**: Report accounts with unusual permissions or exposures.

### 3. **Group Policy Misconfiguration Analysis**
   **Objective**: Identify and rectify misconfigured GPOs.
   - **Step 1**: Analyze GPO settings using PowerView.
   - **Step 2**: Identify weak configurations, such as unrestricted scripts.
   - **Step 3**: Recommend policy changes to harden GPOs.

### 4. **Domain Trust Relationship Mapping**
   **Objective**: Analyze domain trust relationships for exploitation paths.
   - **Step 1**: Query trust relationships using BloodHound or PowerView.
   - **Step 2**: Map potential attack paths to privileged accounts.
   - **Step 3**: Provide actionable insights to secure trust boundaries.

### 5. **LSA Secrets Enumeration**
   **Objective**: Extract sensitive secrets stored in the Local Security Authority (LSA).
   - **Step 1**: Use tools like secretsdump.py to dump LSA secrets.
   - **Step 2**: Parse results for credentials and sensitive data.
   - **Step 3**: Report findings and suggest mitigations.

### 6. **Privilege Escalation Path Analysis**
   **Objective**: Identify and mitigate privilege escalation paths.
   - **Step 1**: Collect AD object permissions with BloodHound.
   - **Step 2**: Analyze potential misconfigurations (e.g., WriteDACL permissions).
   - **Step 3**: Provide remediation steps for high-risk paths.

---

## Recommendations

1. **Regular Auditing**: Continuously monitor AD for anomalies.
2. **Least Privilege Principle**: Minimize user and service permissions.
3. **Defense-in-Depth**: Layered security to mitigate multiple attack vectors.
4. **Tool Familiarization**: Ensure teams are trained on both offensive and defensive tools.
5. **Cross-Team Collaboration**: Facilitate communication between red and blue teams.




To create a comprehensive list of **data enrichment workflows** tailored to **Active Directory (AD) environments**, let’s break down the typical types of data found during offensive operations, the enrichment process, and their practical output. Here’s an organized approach:

---

### **General Format**
| **Input**             | **Process**                                     | **Output**                                   | **Tool(s)**                    |
|------------------------|------------------------------------------------|---------------------------------------------|---------------------------------|
| Data type or source    | Method of enrichment (parsing, cracking, etc.) | Actionable results                          | Relevant tools used in the flow |

---

### **Data Enrichment Flows for AD Environments**

#### **Credential Data**
| **Input**                      | **Process**                                       | **Output**                                                   | **Tool(s)**                 |
|--------------------------------|--------------------------------------------------|-------------------------------------------------------------|-----------------------------|
| NTLM hash                      | Crack with dictionary/rainbow tables             | Cracked password                                            | Hashcat, John the Ripper    |
| SAM/NTDS.DIT dump              | Extract hashes                                   | NTLM hashes, plaintext passwords (with DPAPI decryption)    | Mimikatz, secretsdump.py    |
| Kerberos tickets (.kirbi files)| TGS-REP cracking (Kerberoasting)                 | Plaintext service account passwords                         | Rubeus, Kerberoast          |
| Encrypted DPAPI blobs          | Decrypt using master keys                        | Plaintext data (e.g., saved passwords, secrets)             | SharpDPAPI, CredBandit      |
| LAPS credentials               | Query AD for local admin passwords               | Plaintext local admin credentials                           | LAPS Toolkit, PowerView     |
| Windows Credential Manager     | Dump stored credentials                          | Recovered plaintext credentials                             | SharpCredential, Mimikatz   |
| NTLM challenge-response        | Pass to NTLM relay attack or crack response      | Session hijack or cracked credential                        | Responder, CrackMapExec     |

---

#### **Network Reconnaissance Data**
| **Input**                  | **Process**                                     | **Output**                                   | **Tool(s)**                    |
|----------------------------|------------------------------------------------|---------------------------------------------|---------------------------------|
| SPNs (Service Principal Names) | Query AD for service accounts               | List of services, potential targets for Kerberoasting       | PowerView, BloodHound        |
| Group Policy Objects (GPOs)| Enumerate permissions                          | Misconfigurations or exploitable settings                   | SharpGPOAbuse, BloodHound    |
| Active Directory schema    | Parse ACLs, analyze attack paths               | Identification of attack paths to high-value targets        | BloodHound, ADRecon          |
| Forest trust information   | Analyze inter-forest relationships            | Potential trust exploitation paths                          | PingCastle, ADExplorer       |

---

#### **File Data**
| **Input**                   | **Process**                                     | **Output**                                   | **Tool(s)**                    |
|-----------------------------|------------------------------------------------|---------------------------------------------|---------------------------------|
| SYSVOL directory contents   | Search for embedded credentials in scripts     | Hardcoded plaintext credentials, secrets                      | GPP Password Finder, PowerView |
| Log files                   | Extract and analyze                            | Authentication artifacts, error details                      | Logparser, Splunk             |
| CSV/XML/JSON config files   | Parse for stored credentials or keys           | Extracted secrets, misconfigurations                         | Python scripts, CyberChef     |

---

#### **System and Registry Data**
| **Input**                  | **Process**                                     | **Output**                                   | **Tool(s)**                    |
|----------------------------|------------------------------------------------|---------------------------------------------|---------------------------------|
| Registry hive              | Parse and analyze                              | Stored credentials, autologon keys                          | RegRipper, PowerShell scripts |
| Scheduled tasks            | Enumerate task configurations                  | Exploitable paths (e.g., DLL hijacking opportunities)        | schtasks.exe, PowerShell      |
| Local SAM database         | Extract and crack hashes                       | Local admin credentials                                     | secretsdump.py, pwdump        |

---

#### **Authentication Data**
| **Input**                       | **Process**                                     | **Output**                                   | **Tool(s)**                    |
|---------------------------------|------------------------------------------------|---------------------------------------------|---------------------------------|
| Kerberos TGT                    | Pass-the-ticket or analyze for abuse           | Hijackable sessions, escalated permissions                    | Rubeus, Mimikatz              |
| LDAP queries                    | Dump AD user and group information             | List of potential privilege escalation paths                 | ldapdomaindump, CrackMapExec  |
| PKI certificates                | Enumerate and abuse misconfigurations          | Escalation via ADCS vulnerabilities                          | Certify, PKIAudit             |

---

#### **Miscellaneous Flows**
| **Input**                       | **Process**                                     | **Output**                                   | **Tool(s)**                    |
|---------------------------------|------------------------------------------------|---------------------------------------------|---------------------------------|
| BloodHound ingested AD data     | Analyze graph                                  | Attack paths to domain admin                | BloodHound, SharpHound         |
| Emails (via compromised accounts)| Search inbox for credentials and attachments  | Sensitive data, further compromise options  | MailSniper, Python scripts     |
| PowerShell transcripts          | Analyze logs for suspicious activity           | Detect injected commands, harvested data    | Logparser, Splunk              |

---

### **Your Next Steps**
1. **Expand the Dataset**: Add more combinations for device artifacts, Azure AD integration, and hybrid environments.
2. **Incorporate Detection Countermeasures**: Each input-process-output flow can include ways to detect and mitigate such enrichments for defensive purposes.
3. **Create Visualizations**: Tools like BloodHound or Lucidchart can be used to map out these flows for easier reference.

