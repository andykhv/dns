# Recursive Resolver
- A simple tool that resolves domain names recursively starting at a root server.
- To understand more about [RFC1035](https://www.ietf.org/rfc/rfc1035.txt), I implemented a custom parser for DNS messages.
- It can parse based on [RFC1035](https://www.ietf.org/rfc/rfc1035.txt):
  - DNS header
  - Question resorce records of *IN* type
  - Answer resource records (*A*, *AAAA*)
  - Authority resource records (*NS*)
  - Additional resource records (*A*, *AAAA*)
