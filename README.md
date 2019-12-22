NOTE: no longer hosting

## Deploying with k8s


### 
```

```

# Set up on GCP
1. go to console.cloud.google.com
2. create project and name it
3. kubernetes engine -> create cluster
    - zonal
    - us-central1-a
    - 3-node pool with f1-micro
    - boot disk size to 10 GB
    - enable pre-emptible nodes
    - enable auto-upgrade and auto-repair
    - disable load balancing
    - disable stack driver options
4. create .travis.yml file
5. generate service account
    - IAM & Admin -> Service Accounts
    - click create
    - role kubernetes engine admin
    - create key json
    - this will save service account json file
    - create encrypted json file
    - From app root dir, run
    ```
    docker run -it -v $(pwd):/app ruby:2.3 sh
    cd app
    gem install travis --no-document
    travis -> n (for no install)
    travis login --pro (with github login)
    copy json file into root of project folder
    rename file service-account.json
    travis encrypt-file service-account.json -r jjang4001/myapp --com
    copy given command into the first line of before_install of .travis.yml
    add service-account.json.enc into the repo
    remove service-account.json from repo
    ```
6. add Docker username and password env vars to travis
7. open gcp shell - only need to run once per cluster
    ```
    gcloud config set project <app-id>
    gcloud config set compute/zone <my compute zone i.e. us-central1-a>
    gcloud container clusters get-credentials myapp-cluster
    ```
8. test cluster connection with `kubectl get pods`.
9. in gcp console, (https://helm.sh/docs/using_helm/#quickstart-guide)
    ```
    curl https://raw.githubusercontent.com/helm/helm/master/scripts/get > get_helm.sh
    chmod 700 get_helm.sh
    ./get_helm.sh
    ```
10. set up tiller service account - in gcp, run
    ```
    kubectl create serviceaccount --namespace kube-system tiller
    kubectl create clusterrolebinding tiller-cluster-rule --clusterrole=cluster-admin --serviceaccount=kube-system:tiller
    helm init --service-account tiller --upgrade
    ```
11. ingress-nginx with helm
    ```
    helm install stable/nginx-ingress --name my-nginx --set rbac.create=true
    ```
12. push to master
13. Check that it's deployed by going to services and clicking the ip address in gcp
