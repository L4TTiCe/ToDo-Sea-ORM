# ToDo-Axtix

[![Run in Postman](https://run.pstmn.io/button.svg)](https://god.gw.postman.com/run-collection/10378707-f5013945-08ac-4bba-928e-5e118c871391?action=collection%2Ffork&collection-url=entityId%3D10378707-f5013945-08ac-4bba-928e-5e118c871391%26entityType%3Dcollection%26workspaceId%3Dc350175f-1637-4627-bad9-a3d86ea574a8#?env%5BDefault%5D=W3sia2V5IjoiQkFTRV9VUkwiLCJ2YWx1ZSI6Imh0dHA6Ly9sb2NhbGhvc3Q6NDAwMCIsImVuYWJsZWQiOnRydWUsInR5cGUiOiJkZWZhdWx0Iiwic2Vzc2lvblZhbHVlIjoiaHR0cDovL2xvY2FsaG9zdDo0MDAwIiwic2Vzc2lvbkluZGV4IjowfSx7ImtleSI6IlJFTU9URV9VUkwiLCJ2YWx1ZSI6Imh0dHBzOi8vdG9kby1ub2RlLWludGVsLmhlcm9rdWFwcC5jb20iLCJlbmFibGVkIjp0cnVlLCJ0eXBlIjoiZGVmYXVsdCIsInNlc3Npb25WYWx1ZSI6Imh0dHBzOi8vdG9kby1ub2RlLWludGVsLmhlcm9rdWFwcC5jb20iLCJzZXNzaW9uSW5kZXgiOjF9XQ==)

A simple ToDo HTTP server, wriited in Rust, using Actix Web Framework.

API Documentation: [Postman Link](https://documenter.getpostman.com/view/10378707/UzJPMatX)

## A Simple ToDo App Service

This repository contains a simple backend built using Actix Web Framework, for a
basic ToDo Application.

It uses MongoDB for its Database, and uses the official MongoDB drivers to access the same.

For more information about the TASK, refer to [TASK.md](TASK.md).

### Running the Application

Once you've cloned the repository you can start the service with
```bash
cargo run
```

If you want a spin up a Docker container with the Rust App, together with a MongoDB Docker instance, 
```bash
cargo make docker
```
> Note: 'cargo make' requires the 'cargo-make' crate, which can be installed using 
'cargo install --force cargo-make'. Refer [https://github.com/sagiegurari/cargo-make](https://github.com/sagiegurari/cargo-make) for more information.
