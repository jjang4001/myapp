docker build -t jji421/myapp-client:latest -t jji421/myapp-client:$SHA -f ./client/Dockerfile ./client
docker build -t jji421/myapp-api:latest -t jji421/myapp-api:$SHA -f ./api/Dockerfile ./api

docker push jji421/myapp-client:latest
docker push jji421/myapp-api:latest

docker push jji421/myapp-client:$SHA
docker push jji421/myapp-api:$SHA

kubectl apply -f k8s
kubectl set image deployments/myapp-server-deployment server=jji421/myapp-api:$SHA
kubectl set image deployments/myapp-client-deployment client=jji421/myapp-client:$SHA