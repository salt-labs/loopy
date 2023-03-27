#!/usr/bin/env bash

SCRIPT_DIR=$(dirname "${BASH_SOURCE[0]}")

kubectl apply -f https://raw.githubusercontent.com/metallb/metallb/v0.13.7/config/manifests/metallb-native.yaml

kubectl wait \
	--namespace metallb-system \
	--for=condition=ready pod \
	--selector=app=metallb \
	--timeout=90s

docker network inspect -f '{{.IPAM.Config}}' kind

kubectl apply -f "${SCRIPT_DIR}/config.yaml"

LB_IP=$(kubectl get svc/test -n test -o=jsonpath='{.status.loadBalancer.ingress[0].ip}')

echo "LoadBalancer IP: ${LB_IP}"

for _ in {1..10}; do

	# curl -H "Host: example.com" http://192.0.2.1/
	curl --verbose ${LB_IP}:80

done
