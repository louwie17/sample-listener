## Receiver for Pine A64

This is a simple rust application that receives signals from a rfm69 transmitter, transmitting
moisture, temperature, and humidity data.
This gets added to a SQLite database.

### To Compile

In order to cross compile it for the nano pi neo 2 I created a docker container.
Before compiling make sure that you have the 2 dependencies in the parent folder as well.
```
docker build . -t  sample-listener-container
CONTAINER_ID=$(docker run -v $(dirname `pwd`):/usr/project -i sample-listener-container:latest sh)
/usr/bin/make -C sample-listener
```

### Use it

...

### Dependencies

https://github.com/louwie17/receiver-pine64
https://github.com/friendlyarm/WiringNP

## Author

Lourens Schep
