# Enrichment Flows for Active Directory Data

This document elaborates on the enrichment flows designed to extract and analyze data from Active Directory (AD) environments. It highlights offensive tactics, techniques, and procedures (TTPs), serving as a comprehensive reference for cybersecurity professionals focusing on Active Directory exploitation.

---

## Purpose

Active Directory is a critical component in many enterprise IT environments. Understanding its structure and potential vulnerabilities is key for:

1. **Red Team Operations**: Simulating adversary tactics to identify gaps in defenses.
2. **Threat Modeling**: Understanding how attackers move laterally and escalate privileges.
3. **Offensive Research**: Developing novel attack vectors.

---

## Offensive Enrichment Flows

### General Format
| **Input**             | **Process**                                     | **Output**                                   | **Tool(s)**                    |
|------------------------|------------------------------------------------|---------------------------------------------|---------------------------------|
| Data type or source    | Method of enrichment (parsing, cracking, etc.) | Actionable results                          | Relevant tools used in the flow |

---

### **Credential Data Enrichment**
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

### **Network Reconnaissance Flows**
| **Input**                  | **Process**                                     | **Output**                                   | **Tool(s)**                    |
|----------------------------|------------------------------------------------|---------------------------------------------|---------------------------------|
| SPNs (Service Principal Names) | Query AD for service accounts               | List of services, potential targets for Kerberoasting       | PowerView, BloodHound        |
| Group Policy Objects (GPOs)| Enumerate permissions                          | Misconfigurations or exploitable settings                   | SharpGPOAbuse, BloodHound    |
| Active Directory schema    | Parse ACLs, analyze attack paths               | Identification of attack paths to high-value targets        | BloodHound, ADRecon          |
| Forest trust information   | Analyze inter-forest relationships            | Potential trust exploitation paths                          | PingCastle, ADExplorer       |

---

### **File and Configuration Data**
| **Input**                   | **Process**                                     | **Output**                                   | **Tool(s)**                    |
|-----------------------------|------------------------------------------------|---------------------------------------------|---------------------------------|
| SYSVOL directory contents   | Search for embedded credentials in scripts     | Hardcoded plaintext credentials, secrets                      | GPP Password Finder, PowerView |
| Log files                   | Extract and analyze                            | Authentication artifacts, error details                      | Logparser, Splunk             |
| CSV/XML/JSON config files   | Parse for stored credentials or keys           | Extracted secrets, misconfigurations                         | Python scripts, CyberChef     |

---

### **System and Registry Data**
| **Input**                  | **Process**                                     | **Output**                                   | **Tool(s)**                    |
|----------------------------|------------------------------------------------|---------------------------------------------|---------------------------------|
| Registry hive              | Parse and analyze                              | Stored credentials, autologon keys                          | RegRipper, PowerShell scripts |
| Scheduled tasks            | Enumerate task configurations                  | Exploitable paths (e.g., DLL hijacking opportunities)        | schtasks.exe, PowerShell      |
| Local SAM database         | Extract and crack hashes                       | Local admin credentials                                     | secretsdump.py, pwdump        |

---

### **Authentication and Exploitation Flows**
| **Input**                       | **Process**                                     | **Output**                                   | **Tool(s)**                    |
|---------------------------------|------------------------------------------------|---------------------------------------------|---------------------------------|
| Kerberos TGT                    | Pass-the-ticket or analyze for abuse           | Hijackable sessions, escalated permissions                    | Rubeus, Mimikatz              |
| LDAP queries                    | Dump AD user and group information             | List of potential privilege escalation paths                 | ldapdomaindump, CrackMapExec  |
| PKI certificates                | Enumerate and abuse misconfigurations          | Escalation via ADCS vulnerabilities                          | Certify, PKIAudit             |

---

### **Miscellaneous Flows**
| **Input**                       | **Process**                                     | **Output**                                   | **Tool(s)**                    |
|---------------------------------|------------------------------------------------|---------------------------------------------|---------------------------------|
| BloodHound ingested AD data     | Analyze graph                                  | Attack paths to domain admin                | BloodHound, SharpHound         |
| Emails (via compromised accounts)| Search inbox for credentials and attachments  | Sensitive data, further compromise options  | MailSniper, Python scripts     |
| PowerShell transcripts          | Analyze logs for suspicious activity           | Detect injected commands, harvested data    | Logparser, Splunk              |

---

## Recommendations

1. **Adopt Offense-Inspired Defense**: Use offensive insights to proactively harden defenses.
2. **Expand Offensive Toolkits**: Regularly explore new TTPs and integrate them into testing methodologies.
3. **Focus on Privilege Escalation**: Develop expertise in identifying misconfigurations and exploiting trust relationships.
4. **Continuous Education**: Stay updated on the latest Active Directory exploits and techniques.
5. **Red Team Collaboration**: Enhance cross-team operations to ensure defensive measures evolve with offensive advancements.

---

This document emphasizes offensive strategies for exploiting Active Directory environments and serves as a foundational guide for professionals focusing on Active Directory attack vectors.

