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

Life Cycles of a Socket
- A socket is created
- socket is then binded to an ip address and a port
- socket then starts listening for connection from a client
- when a client socket connects to the socket, it accepts the connection and creates a new socket to represent that connection with the client
- the former socket then goes on to wait for other connections from other clients repeating the step immediately above

### Questions
- What is the ip address and the port of the new created socket that represents a connection with a client
  - ANS: Every Socket is identified by a 5 tuple value
    - type of the socket: TCP or UDP
    - local port
    - local ip address
    - peer port (optional for UDP)
    - peer ip address (optional for UDP)
    
    so even though the newly created socket would share the same local port and local ip address
    the different peer port and ip address would make them disquishable to the operating system

- Can multiple Sockets exist in an OS for the same Ip address and port
  - ANS: The Association (socket showing a connection between a client and server socket are unique) 


- Server - HttpServer: This is the back bone supporting the application, handles the transport layer of communication
- Application - App: Enscapulates all application logic, middlewares, routing, request handlers, the app handles the connection request and returns a response

- Endpoint - Route: the route method on the App allow us to specify different routes for our application
  - the route method takes in 2 values, a path string and an instance of Route Struct
  - Route combines a handler and a set of guards, guards specify conditions a request must satisfy in other to match and be passed over to the handler
    - Route with guard example -> Route::new().guard(guard::Get()) 
    - this above Route can be written as a short-cut web::get().to(handler_func)
