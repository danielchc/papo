# papo
Create small development environments without having to install anything on the local machine

## Build
```
cargo build
```
## Run interactive container
If Docker host isn't locally installed you must define ``DOCKER_HOST`` variable. Ex.
```
export DOCKER_HOST="192.168.0.2:2375"
```
Next,
```
papo start "debian:stable"
```

## ROADMAP
- [x] Run container from command line
- [ ] Catch error in Docker connection
- [ ] Read environments from file
- [ ] Allow auto install dependencies
- [ ] Mount local directories
- [ ] Local settings file
- [ ] Catch Ctrl+C


