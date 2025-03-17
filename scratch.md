# script
INGRESS_GATEWAY_IP=$(kubectl get service asm-ingressgateway -n ingress-gateway -o jsonpath='{.status.loadBalancer.ingress[0].ip}')

cargo build --release --target wasm32-wasip1

kubectl -n ingress-gateway delete configmap wasm
kubectl -n ingress-gateway create configmap wasm --from-file target/wasm32-wasip1/release/custom_error_message_wasm_status_code.wasm
kubectl -n ingress-gateway rollout restart deployment/asm-ingressgateway

curl -s -H "Host: workload-1.example.com" http://$INGRESS_GATEWAY_IP/workload-1/ -v

kubectl -n ingress-gateway delete configmap wasm
kubectl -n ingress-gateway create configmap wasm --from-file target/wasm32-wasip1/release/custom_error_response_wasm.wasm
kubectl -n ingress-gateway rollout restart deployment/asm-ingressgateway



# wss setup

# label namespace 
kubectl label namespace default istio-injection=enabled --overwrite
kubectl rollout restart deployment echoserver

kubectl apply -f custom-error-message-lua-filter/starting.yaml
kubectl delete -f custom-error-message-lua-filter/starting.yaml

kubectl apply -f ws-sample/

wscat -c ws://34.57.90.31/ws-sample/

# try WASM 
cd custom-error-message-wasm-status-code/
kubectl -n ingress-gateway create configmap wasm --from-file target/wasm32-wasip1/release/custom_error_message_wasm_status_code.wasm


kubectl -n ingress-gateway rollout restart deployment asm-ingressgateway
kubectl -n ingress-gateway apply -f ef.yaml
kubectl -n ingress-gateway delete -f ef.yaml


kubectl apply -f custom-error-message-lua-filter/ignore-ws-path.yaml
kubectl delete -f custom-error-message-lua-filter/starting.yaml

# test gRPC unary 
kubectl create ns whereami-grpc
kubectl label namespace whereami-grpc istio-injection=enabled --overwrite

kubectl apply -k whereami-grpc/variant/

kubectl apply -f whereami-grpc/whereami-grpc-vs.yaml

grpcurl -plaintext -H "Host: grpc.example.com" 34.57.90.31:80 whereami.Whereami.GetPayload | jq .
curl -s -H "Host: workload-1.example.com" http://34.57.90.31/workload-1/ -v
curl -s -H "Host: workload-1.example.com" http://34.57.90.31/workload-123/

kubectl apply -f custom-error-message-lua-filter/improved-wss-handling.yaml

grpcurl -H "Host: your-domain.com" -d '{"field1": "value1", "field2": 123}' <IP_ADDRESS>:<PORT> <SERVICE_NAME>/<METHOD_NAME>
grpcurl -H "Host: grpc.example.com" -authority grpc.example.com -plaintext 34.57.90.31:80 whereami.Whereami/GetPayload
grpcurl -authority grpc.example.com -plaintext 34.57.90.31:80 whereami.Whereami/GetPayload


# testing new logging setup
kubectl apply -f custom-error-message-lua-filter/starting.yaml