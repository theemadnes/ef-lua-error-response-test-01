# ef-lua-error-response-test-01
testing custom error responses via Lua EnvoyFilters

### set up ingress gateway

```
kubectl create ns ingress-gateway

# label the namespace to perform sidecar injection
kubectl label namespace ingress-gateway istio-injection=enabled

# apply ingress gateway YAML
kubectl apply -n ingress-gateway -k ingress-gateway/
```

### deploy first workload (echo-server)

```
kubectl create ns workload-1
kubectl label namespace workload-1 istio-injection=enabled
kubectl apply -n workload-1 -f workload-1/
```

### attempt to call first workload

```
# capture IP of ingress gateway service
INGRESS_GATEWAY_IP=$(kubectl get service asm-ingressgateway -n ingress-gateway -o jsonpath='{.status.loadBalancer.ingress[0].ip}')

# call the first workload 
curl -s -H "Host: workload-1.example.com" http://$INGRESS_GATEWAY_IP/workload-1 | jq
```