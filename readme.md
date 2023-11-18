# Basic Http Server
# Paul Mana

## Goals

Learn more deeply about the server interactions and considerations in its arch
itecture. Hopefuly dog food the basic get only version for a nice simple static
host server.

Also the process of reading and acting on an RFC is a big part of the learnings
I hope to take away from this. They are be borning, dense and full of jargon. I
would like pratice at this.

## TODO

- Parse the html request parts

- Return the parse request

- Multi threading of requests maybe tokio? or simular

- Add logging

- Document code when its in a more stable state

## Browers

Fire fox appears to be sending 9 get requests when I hit the server, at first
I thought rust was just looping over the same request over and over. But it see
ms that its the browers.


