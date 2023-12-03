# Performant SMTP Blackhole

## Overview

The goal is to build a performant SMTP receiver. 
While it may only process incoming mail minimally, it's not designed to relay it in any traditional sense. 
The intended destination could be Redis/Memcached or some other fast storage solution. At the same time, 
I might try implementing a similar thing in pure C to compare its performance with the Rust implementation and decide 
on the best way forward. 
Additionally, I'm looking to learn more about the Rust language through this process.

## Requirements
- Rust
- OpenSSL (for generating self-signed certificates if needed)

## Configuration
The application uses two environment variables to configure the listening ports:

- `SMTP_LISTENERS`: Comma-separated list of addresses and ports for SMTP (e.g., `127.0.0.1:25,127.0.0.1:587`).
- `SMTPS_LISTENERS`: Comma-separated list of addresses and ports for SMTPS (e.g., `0.0.0.0:465`).
- Optional: `CERT_PATH` and `CERT_PASSWORD`: required for SMTPS listeners. Dummy self-signed cert could be generated using `generate_dummy_ssl_cert.sh` from scripts folder.

## Contributions

Contributions to this project are welcome. 

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

Apache License 2.0 is a permissive open source license that allows you to freely use, modify, distribute, and sell your own products that include Apache 2.0 licensed software, without worrying about royalties.

