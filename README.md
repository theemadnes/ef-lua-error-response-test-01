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
curl -s -H "Host: workload-1.example.com" http://$INGRESS_GATEWAY_IP/workload-1/ | jq

# test 404
curl -s -H "Host: workload-1.example.com" http://$INGRESS_GATEWAY_IP/workload-1/?echo_code=404 -v | jq

# test 502
curl -s -H "Host: workload-1.example.com" http://$INGRESS_GATEWAY_IP/workload-1/?echo_code=502 -v | jq
```

### attempt to add a custom error response

```
kubectl apply -f custom-error-message-lua-filter/starting.yaml

# test 404
curl -s -H "Host: workload-1.example.com" http://$INGRESS_GATEWAY_IP/workload-1/?echo_code=404 -v

# test 503
curl -s -H "Host: workload-1.example.com" http://$INGRESS_GATEWAY_IP/workload-1/?echo_code=503 -v
```

### seeing some old artifacts from prior tests so restarting ingress gateway pods

```
kubectl -n ingress-gateway rollout restart deployment/asm-ingressgateway
```

### setting up WASM example

```
cargo new --lib custom-error-response-wasm
# doing this within custom-error-response-wasm folder
cd custom-error-response-wasm/
rustup target add wasm32-wasi
cargo build --release --target wasm32-wasi 
# attempting to add newer target
rustup target add wasm32-wasip1
cargo build --release --target wasm32-wasip1

# remove Lua filter
kubectl delete -f custom-error-message-lua-filter/

# create configmap to reference WASM package
kubectl -n ingress-gateway create configmap wasm --from-file target/wasm32-wasip1/release/custom_error_response_wasm.wasm
kubectl -n ingress-gateway delete configmap wasm
kubectl -n ingress-gateway create configmap wasm --from-file target/wasm32-wasi/release/custom_error_response_wasm.wasm

# now edit the ingress gateway deployment to include the following annotations
annotations:
    # sidecar.istio.io/userVolume: '{"wasm":{"configMap":{"name":"wasm"}}}'
    sidecar.istio.io/userVolume: '{"wasm":{"configMap":{"name":"wasm"},"items:[{"key":"custom_error_response_wasm.wasm","path":"/custom_error_response_wasm.wasm}]}}'
    sidecar.istio.io/userVolumeMount: '{"wasm":{"mountPath":"/wasm","readOnly":true}}'
# this will restart the pods

# apply wasm filter
kubectl apply -f ef.yaml
INGRESS_GATEWAY_IP=$(kubectl get service asm-ingressgateway -n ingress-gateway -o jsonpath='{.status.loadBalancer.ingress[0].ip}')

# test the call
curl -s -H "Host: workload-1.example.com" http://$INGRESS_GATEWAY_IP/workload-1/ -v
```