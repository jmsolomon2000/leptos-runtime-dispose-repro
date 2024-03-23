cargo leptos build --release -vv
sudo docker build . -t localhost:32000/runtime-dispose-repro:registry-1
sudo docker push localhost:32000/runtime-dispose-repro:registry-1
microk8s kubectl apply -f deployment.yml
