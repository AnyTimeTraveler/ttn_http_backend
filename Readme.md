# TTN HTTP Backend

A simple program that can act as the backend for Applications on [The Things Network](https://www.thethingsnetwork.org/)

This project uses the [The Things Network HTTP Integration](https://www.thethingsnetwork.org/docs/applications/http/) to communicate with TTN.

This project is intended as a starting point.
It currently just prints received messages and sends "Hello device" messages in reply.

## Requirements

The HTTP Integration requires that you have a public IP address that it can send requests to.

## Configuration

A configuration file will automatically be created with example values and stored at `~/.config/ttn_http_backend/ttn_http_backend.toml`.

## License

MIT License
