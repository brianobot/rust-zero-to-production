# Rust Zero to Production


## Concepts:
- Socket: 

#[youtube video reference](https://www.youtube.com/watch?v=D26sUZ6DHNQ) 

A socket is an abstraction created by the operating system to act as an endpoint between communication 
between 2 processes or 2 machines. it wraps an ip address and a port. if you think of it as a phone
the phone number is the combination of the ip address and the port to get the correct socket.

When a program creates a socket, it is equivalent to obtaining a phone which would be later used for
communication, but at the point of creation (passive open), no number is dialed on the phone

There are 2 types of sockets
  - TCP Sockets: Ordered Data Transmission
  - UDP Sockets: Unordered Data Transmission