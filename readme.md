# First docker experiment

Test for working with docker container.
A mariadb database will be hosted on a docker container.  

This programm will be build into a docker image using BUILD.
The image is made from debian:stable-slim.
A multi-stage build process was used.  

The program is only for testing how docker images and containers work and how they communicate with each other and with the host.  

This is a cli tool with which some database stuff can be done.  

---  

## Usage  

1. Create a network which is needed so that the database container and this program container cancommunicate with eachother.  

`code` docker network create -d bridge my_test_network  

2. Create an mysql / mariadb container.  

`code` docker run -d -p 5000:3306 --network my_test_network --name test_db --env MARIADB_ROOT_PASSWORD=default_root_password --env MARIADB_USER=default_user --env MARIADB_PASSWORD=test_password_for_default_user mariadb:latest  

3. Get IP of database container. Do this everytime the database container is started.  

`code` docker inspect test_db  

4. Build the image for this program  

`code` docker run -i --network my_test_network --name sqlx-test --env-file ./.env docker-sqlx  

--env-file ./.env is needed because the docker builder isn't copying .env files.  

5. Create a container from the build image  

`code` docker build -t docker-sqlx -f ./dockerfile .  

6. Use the program  

---  

## TODO

- [ ] Change connection building code to be changed at runtime
  - [ ] Put default starting configuration into env
  - [ ] Implement database connecting -> Changing connection string
  - [ ] Implement user switching -> Changing connection string
- [ ] Using docker compose so that i don't have to inspect every time the database is started.
- [ ] Implement useful stuff to do in a database. (e.g. Create Table, Crud, ...)
