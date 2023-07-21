# MachaonWeb

## A distributed computing network for Machaon

MachaonWeb is an online platform that performs distributed caching and computation of user requests. It is a project written in Rust (back folder) & React/Bootstrap (front folder) that provides online access to Machaon, 
a method for identifying and profiling structural similarities between protein structures. The method is described in [https://www.nature.com/articles/s42003-023-05076-7](https://www.nature.com/articles/s42003-023-05076-7). 

MachaonWeb securely communicates with instances of Machaon via mutual TLS gRPC.

For more details, you can also visit [https://machaonweb.com](https://machaonweb.com) where this system is online.
   
<br/>
   
## Installation

The deployment mainly relies on Docker (https://docs.docker.com/engine/install/ubuntu/) and Docker Compose (https://docs.docker.com/compose/install/). Node Package Manager (NPM) is also needed to build the frontend part of MachaonWeb and it is bundled with Node JS (https://nodejs.org/en). You can follow the steps below:

- Build the frontend using NPM by running `npm run build` command inside the `front` folder. A `build` folder will be created, containing the required files for the UI of the platform. If you want to use Google Captcha & Google Analytics, you can generate your own keys and place them in `front/.env` file.

- Install MariaDB using the official Docker image (https://hub.docker.com/_/mariadb): `docker run --name web-db -e MYSQL_ROOT_PASSWORD=<YOUR_ROOT_PASSWORD> -p <DBPORT-external>:<DBPORT-internal> -d mariadb:latest`

- Create a database user that will be used by MachaonWeb:

    `CREATE DATABASE machaon CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci;` 

    `GRANT ALL PRIVILEGES ON machaon.* TO '<DB_USERNAME>'@'%' IDENTIFIED BY "<DB_USER_PASSWORD>";`

    DBeaver can be used to conveniently access MariaDB : [https://dbeaver.io/download/](https://dbeaver.io/download/)

    Run the SQL commands manually in `2023-02-18-163221_create_db/up.sql`. (Creating database with Diesel CLI will overwrite `back/schema.rs`, which contains custom struct definitions). The last SQL command adds a computing node in the network, so edit the values accordingly. You can run a computing node in the same machine that hosts a root node by carefully assigning the various ports.


- Running the platform with HTTPS & mutual TLS enabled is the default setting but you can uncomment & comment parts in the code to override this configuration for debugging purposes (`back/src/main.rs, back/src/grpc/mod.rs, back/src/rest/mod.rs`). 

- You can generate the self-signed certificates for mutual TLS using the `back/mTLS/generate-keys.sh` script that is based on this example: [https://github.com/rustls/rustls/tree/main/test-ca](https://github.com/rustls/rustls/tree/main/test-ca). Edit the script according to your needs, node0 is the root node and node1 is a computing node. You can add another node by duplicating the commands for node1. Here is an example of configuration:

    * Generate the certificates using the local domains as CNs and setup SANs like: DNS.1 = bob.localdomain

    * Add the local root CA to the trust store of each VM (copy the root CA certificate to `/usr/local/share/ca-certificates` with .crt extension [important, otherwise the file is ignored] and run `update-ca-certificates`)

    * Here is an example of a local domain for gRPC call: https://bob.localdomain:54321 

- The nodes are managed manually in the database (`nodes` table).

- Edit the environment variables in `back/docker-compose.yml` according to your needs and the host system's setup. Place the required files (frontend files, certificates) into the locations that you designated in the yaml file.

- Run `docker compose up -d` inside `back` folder in order to create the backend image based on the official Rust Docker image (https://hub.docker.com/_/rust)

- Create an internal network for the communication between the database and backend container:   
  `docker network create InternalMachaonweb`  
  `docker network connect InternalMachaonweb web-db`  
  `docker network connect InternalMachaonweb <BACKEND_CONTAINER_NAME>`

- When everything is ready, you can enter into the backend container and run the compiled executable in the `target/release` folder. These commands will be useful:  
    `docker ps -a`  
    `docker exec -i -t <CONTAINER-ID> /bin/bash`

