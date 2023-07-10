# Lecture 1 – Consistent Hashing

- Lecture Notes: https://web.stanford.edu/class/cs168/l/l1.pdf

# Summary

We'll build a server and client in rust. The servers will form a distribuited cache of bcrypt hashes. The client will
be able to send a password to the server and get back the bcrypt hash of that password.

If there's a cache hit the server will respond immediately. If there's a cache miss the server will have to
perform the bcrypt hash and then cache the result. We'll pick a bcrypt cost factor that makes the hash take
a couple of seconds to compute.

The client will pick the servers to send the request to using consistent hashing of the password.

We'll deploy multiple instances of the server using Docker. Clients will get the server addresses from config.

# Tasks

[] run in server mode if `--server` flag present
[] server must listen to port 8989
[] server must respond to 'PING' with 'PONG'
[] client should have ping command
