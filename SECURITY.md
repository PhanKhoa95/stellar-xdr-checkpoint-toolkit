# Security Policy

## Reporting a Vulnerability

Please report cryptographic or XDR parsing vulnerabilities privately via GitHub Security Advisories on this repository.

## Cryptographic Invariants

- All SHA-256 digests over `LedgerHeader` XDR payloads are compared in constant time or exact 32-byte array equality.
- Uncompressed XDR streams are bounded prior to RFC 4506 frame allocation.
