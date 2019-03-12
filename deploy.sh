docker build -t jji421/myapp-client:latest -f ./client/Dockerfile ./client
docker build -t jji421/myapp-api:latest -f ./api/Dockerfile ./api
docker push jji421/myapp-client:latest
docker push jji421/myapp-api:latest
 