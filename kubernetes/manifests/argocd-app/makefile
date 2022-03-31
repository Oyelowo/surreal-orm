setup:
	cp -R secret-unsealed.example.yaml secret-unsealed.yaml
	echo "Copy created🎉: edit the new argocd-secret file to include the credentials and run the next coommand"

create:
	kubeseal <secret-unsealed.yaml -o yaml >./ready/secret-sealed.yaml --namespace argocd