
ips=$(kubectl get pods -l app=redis-cluster -o jsonpath='{range.items[*]}{.status.podIP}:6379 ')
kubectl exec -it redis-cluster-0 -- redis-cli --cluster create --cluster-replicas 1 $ips