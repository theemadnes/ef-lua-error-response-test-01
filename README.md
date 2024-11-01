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

### create WASM plugin based on HTTP status code

> note: for this to work, you need to modify the ingress gateway deployment to mount the wasm binary in the deployment spec, like so:

```
# at container level in deployment config for ingress gateway
volumeMounts:
- mountPath: /wasm
  name: wasm
  readOnly: true
# at pod level in deployment config for ingress gateway
volumes:
- name: wasm
  configMap:
    name: wasm
```

```
# create plugin lib
cargo new --lib custom-error-message-wasm-status-code
cd custom-error-message-wasm-status-code
cat <<EOF >> Cargo.toml
proxy-wasm = "0.2"
log = "0.4"
[lib]
crate-type = ["cdylib"]
[profile.release]
lto = true
opt-level = 3
codegen-units = 1
panic = "abort"
strip = "debuginfo"
EOF

# set up src/lib.rs
# compile src/lib.rs
cargo build --release --target wasm32-wasip1

# create configmap with WASM plugin
kubectl -n ingress-gateway create configmap wasm --from-file target/wasm32-wasip1/release/custom_error_message_wasm_status_code.wasm
# apply envoy filter
kubectl -n ingress-gateway apply -f ef.yaml
# redeploy ingress gateway pods
kubectl -n ingress-gateway rollout restart deployment/asm-ingressgateway

# test
INGRESS_GATEWAY_IP=$(kubectl get service asm-ingressgateway -n ingress-gateway -o jsonpath='{.status.loadBalancer.ingress[0].ip}')
curl -s -H "Host: workload-1.example.com" http://$INGRESS_GATEWAY_IP/workload-1/ -v # this should work fine
curl -s -H "Host: workload-1.example.com" http://$INGRESS_GATEWAY_IP/workload-123/ -v # this should give you a 404 
```
