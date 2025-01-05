# Enrichment Flows for Active Directory Data

> **Purpose**  
> This document provides an overview of how to extract and analyze data from Active Directory (AD) environments. It highlights common offensive tactics, techniques, and procedures (TTPs) relevant to cybersecurity professionals.

---

## Table of Contents

1. [Why Active Directory Matters](#why-active-directory-matters)  
2. [Offensive Enrichment Flows](#offensive-enrichment-flows)  
   1. [General Format](#general-format)  
   2. [Credential Data Enrichment](#credential-data-enrichment)  
   3. [Network Reconnaissance Flows](#network-reconnaissance-flows)  
   4. [File and Configuration Data](#file-and-configuration-data)  
   5. [System and Registry Data](#system-and-registry-data)  
   6. [Authentication and Exploitation Flows](#authentication-and-exploitation-flows)  
   7. [Miscellaneous Flows](#miscellaneous-flows)  
3. [Recommendations](#recommendations)  

---

## Why Active Directory Matters
Active Directory is the heart of identity management in many enterprise IT environments. Gaining insight into how attackers exploit AD is crucial for:
1. **Red Team Operations**: Emulating advanced threats to find defensive weaknesses.
2. **Threat Modeling**: Identifying how attackers pivot and escalate privileges.
3. **Offensive Research**: Creating and testing new AD-focused attack vectors.

---

## Offensive Enrichment Flows

> **Note:**  
> The following tables outline common inputs, processes, and outputs for each AD enrichment flow. In a real-world engagement, multiple flows might be combined for more advanced campaigns.

### General Format

| **Input**             | **Process**                                     | **Output**                        | **Tool(s)**                  |
|-----------------------|-------------------------------------------------|-----------------------------------|------------------------------|
| Data type or source   | Method of enrichment (e.g., parsing, cracking)  | Actionable results                | Relevant tools used          |

---

### Credential Data Enrichment

| **Input**                      | **Process**                                               | **Output**                                                   | **Tool(s)**                 |
|--------------------------------|----------------------------------------------------------|-------------------------------------------------------------|-----------------------------|
| NTLM hash                      | Crack with dictionary/rainbow tables                     | Cracked password                                            | Hashcat, John the Ripper    |
| SAM/NTDS.DIT dump              | Extract hashes; optionally decrypt using DPAPI           | NTLM hashes, plaintext passwords                            | Mimikatz, secretsdump.py    |
| Kerberos tickets (.kirbi files)| TGS-REP cracking (Kerberoasting)                         | Plaintext service account passwords                         | Rubeus, Kerberoast          |
| Encrypted DPAPI blobs          | Decrypt using master keys                                | Plaintext data (e.g., saved passwords, secrets)             | SharpDPAPI, CredBandit      |
| LAPS credentials               | Query AD for local admin passwords                       | Plaintext local admin credentials                           | LAPS Toolkit, PowerView     |
| Windows Credential Manager     | Dump stored credentials                                  | Recovered plaintext credentials                             | SharpCredential, Mimikatz   |
| NTLM challenge-response        | Pass to NTLM relay attack or crack response              | Session hijack or cracked credential                        | Responder, CrackMapExec     |

---

### Network Reconnaissance Flows

| **Input**                  | **Process**                                     | **Output**                                   | **Tool(s)**                    |
|----------------------------|------------------------------------------------|---------------------------------------------|---------------------------------|
| SPNs (Service Principal Names) | Query AD for service accounts               | List of services, potential Kerberoasting targets            | PowerView, BloodHound         |
| Group Policy Objects (GPOs)| Enumerate permissions                          | Misconfigurations or exploitable settings                   | SharpGPOAbuse, BloodHound     |
| AD schema                  | Parse ACLs, analyze attack paths               | Potential routes to high-value targets                      | BloodHound, ADRecon           |
| Forest trust information   | Analyze inter-forest relationships            | Potential trust exploitation paths                          | PingCastle, ADExplorer        |

---

### File and Configuration Data

| **Input**                   | **Process**                                     | **Output**                                             | **Tool(s)**                    |
|-----------------------------|------------------------------------------------|-------------------------------------------------------|---------------------------------|
| SYSVOL directory contents   | Search for embedded credentials in scripts     | Hardcoded plaintext credentials, secrets              | GPP Password Finder, PowerView |
| Log files                   | Extract and analyze                            | Authentication artifacts, error details              | Logparser, Splunk             |
| CSV/XML/JSON config files   | Parse for stored credentials or keys           | Extracted secrets, misconfigurations                  | Python scripts, CyberChef     |

---

### System and Registry Data

| **Input**                  | **Process**                                     | **Output**                                   | **Tool(s)**                    |
|----------------------------|------------------------------------------------|---------------------------------------------|---------------------------------|
| Registry hive              | Parse and analyze                              | Stored credentials, autologon keys                          | RegRipper, PowerShell scripts |
| Scheduled tasks            | Enumerate task configurations                  | Exploitable paths (e.g., DLL hijacking opportunities)        | schtasks.exe, PowerShell      |
| Local SAM database         | Extract and crack hashes                       | Local admin credentials                                     | secretsdump.py, pwdump        |

---

### Authentication and Exploitation Flows

| **Input**                       | **Process**                                     | **Output**                                   | **Tool(s)**                    |
|---------------------------------|------------------------------------------------|---------------------------------------------|---------------------------------|
| Kerberos TGT                    | Pass-the-ticket or analyze for abuse           | Hijackable sessions, escalated permissions                    | Rubeus, Mimikatz              |
| LDAP queries                    | Dump AD user and group information             | Potential privilege escalation paths                          | ldapdomaindump, CrackMapExec  |
| PKI certificates                | Enumerate and exploit misconfigurations        | Potential escalation via ADCS vulnerabilities                 | Certify, PKIAudit             |

---

### Miscellaneous Flows

| **Input**                       | **Process**                                     | **Output**                                      | **Tool(s)**                    |
|---------------------------------|------------------------------------------------|------------------------------------------------|---------------------------------|
| BloodHound ingested AD data     | Analyze graph                                  | Potential attack paths to domain admin          | BloodHound, SharpHound         |
| Emails (via compromised accounts)| Search inbox for credentials and attachments  | Sensitive data, further compromise options      | MailSniper, Python scripts     |
| PowerShell transcripts          | Analyze logs for suspicious activity           | Detection of malicious commands, exfil data     | Logparser, Splunk              |

---

## Recommendations

1. **Adopt Offense-Inspired Defense**  
   Leverage these flows to identify common weaknesses and proactively harden your AD environment.
2. **Expand Offensive Toolkits**  
   Stay current with emerging TTPs and incorporate them into your testing and red team exercises.
3. **Focus on Privilege Escalation**  
   Pay particular attention to trust relationships and misconfigurations that enable lateral movement.
4. **Continuous Education**  
   Keep up with the latest AD exploits and infiltration strategies in the cybersecurity community.
5. **Red Team Collaboration**  
   Work closely with red teams to continually evolve your defensive measures as offensive capabilities grow.

