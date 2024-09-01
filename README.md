# Thex

Thex stands for Thought Exchange. It is a message encryption system designed to facilitate secure communication. Thex allows users to encrypt and decrypt messages, ensuring that sensitive information remains private and protected.

## Objective

The goal of this repository is to explore message encryption by offering a simple implementation for secure communication. It allows users to encrypt and decrypt messages, ensuring safe, free, and independent communication.

At this stage, the implementation uses a server to handle communication. Encrypted messages are delivered to the server and deleted immediately once they are delivered to the recipient. The recipient must request the messages, potentially using a code to access them. This approach simplifies the initial implementation.

In the future, it would be ideal to implement a peer-to-peer (P2P) system similar to how torrenting or blockchain nodes work. For now, the intermediate storage can be handled using Elasticsearch.

## How It Works

- Generate RSA Key Pair: Create a pair of RSA keys (private and public)
- Encrypt Message: Encrypt a message using the public key.
- Decrypt Message: Decrypt the message using the private key.

## Future Plans

- P2P Communication: Transition to a P2P model for decentralized communication.
- Code for Retrieval: Implement a secure code-based retrieval system for messages.
- Intermediate Storage: Use Elasticsearch for temporary storage of messages.
- AES Encryption: Integrate AES for message encryption based on the following approach:
  - RSA for Key Exchange: Use RSA to securely exchange symmetric keys.
  - AES for Message Encryption: Use AES to encrypt and decrypt the actual message content. This hybrid approach combines the benefits of both public-key (RSA) and symmetric-key (AES) cryptography.
