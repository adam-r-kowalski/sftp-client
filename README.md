# sftp-client

## Setup Docker

Install docker for your desired platform by following the link [here](https://docs.docker.com/install/).

## Setup Docker Compose

Install docker compose for your desired platform by following the link [here](https://docs.docker.com/compose/install/).

## Clone The Repository

Using ssh

`git clone git@github.com:adam-r-kowalski/sftp-client.git`

Using https

`git clone https://github.com/adam-r-kowalski/sftp-client.git`

## Spin up the containers!

`docker-compose up -d`

## Attach to the container

`docker ps -a`

You should see something like this

```
CONTAINER ID        IMAGE                     COMMAND               CREATED             STATUS              PORTS               NAMES
0b2985ebd61e        adamkowalski/rust-cmake   "bash"                5 seconds ago       Up 3 seconds                            sftp-client_client_1
5fb7471e8578        gotechnies/alpine-ssh     "/usr/sbin/sshd -D"   5 seconds ago       Up 2 seconds        22/tcp              sftp-client_server_1
```

Make note of the CONTAINER ID for the adamkowalski/rust-cmake image, in this case `0b2985ebd61e`

Connect to the container by running

`docker attach 0b2985ebd61e`

You will now be greeted by a prompt saying

`root@0b2985ebd61e:/client#`

This means you are inside the docker container!

`exit` will take you out of the container

Then type `docker-compose down` in the project directory to shut down all containers

## Running the code

`cargo run`

## Testing the code

`cargo test`

## Editing the code

Within the container emacs is already installed, and it is already configured for editing rust code

Beyond that you can use any editor you like. Here are some recommendations

- [Visual Studio Code](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust)
- [IntelliJ](https://intellij-rust.github.io/)

