`kubectl delete all --all -n {namespace}`

```sh
(
NAMESPACE=your-rogue-namespace
rm temp.json
kubectl proxy &
kubectl get namespace $NAMESPACE -o json |jq '.spec = {"finalizers":[]}' >temp.json
curl -k -H "Content-Type: application/json" -X PUT --data-binary @temp.json 127.0.0.1:8001/api/v1/namespaces/$NAMESPACE/finalize
kill $(lsof -t -i:8001)
)
```
