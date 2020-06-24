# hackIT_ng (WIP)
Programing challenge site for IT students at Chalmers
by digIT20

## BUILD
1. Make sure you have a docker daemon running
2. Run `docker-compose up --build` in the root directory
(This will take a few minutes if this is the first time it runs or the dependencies have changed)
Once you see `Running target/release/hackIT` you can use the project.

## USAGE
- Enter localhost:1337 in your browser
- Enter localhost:1337/completions to test db
- Enter localhost:8080 to administrate the db

## CONFIG
all config values are in `.env` in the root directory
here you can change DB host,password,user etc

