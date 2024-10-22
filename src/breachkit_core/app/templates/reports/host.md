# {{ operation_name }} Report

**Target:** {{ target }}

**Status:** {{ status }}

## Findings
{% for finding in findings %}
- **Service:** {{ finding.service }}
- **Port:** {{ finding.port }}
- **Details:** {{ finding.details }}
{% endfor %}
