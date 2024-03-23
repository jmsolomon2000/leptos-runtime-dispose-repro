# Reproduce [expected context of type "leptos_axum::ResponseOptions" to be present]

## Pre-requisites
Microk8s (https://microk8s.io/) was used as the Kubernetes test environment on an Ubuntu 23.10 machine. Any K8s environment should be sufficient. 
The issue currently occurs on my fly.io 1-cpu-256mb-memory VM as well. 

- MicroK8s or equivalent (https://microk8s.io/)
- MicroK8s local registry feature must be enabled for the Dockerfile and build_and_deploy.sh to work (https://microk8s.io/docs/registry-built-in)
- MicroK8s dashboard feature must be enabled for the Dockerfile and build_and_deploy.sh to work (`microk8s enable dashboard`)


## Running

1. Run the `build_and_deploy.sh` script. 
2. Go to the MicroK8s dashboard and get the IP address of the deployed service.
3. Navigate to the deployed service's IP adress on port 3000.
4. Press the button "click" to reproduce.

## Implementation

Upon pressing the "click" button, three server fns are called, which randomly thread sleep and `await` before returning. 

