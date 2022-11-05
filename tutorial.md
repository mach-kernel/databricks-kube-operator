---
description: An example GitOps recipe
---

# Tutorial

This repository contains a [PySpark example job](https://github.com/mach-kernel/databricks-kube-operator/blob/master/examples/job.py) (how did you guess it was word count?) that we are going to operationalize using Helm and databricks-kube-operator. You can follow along with a local [minikube](https://minikube.sigs.k8s.io/docs/) cluster, or use in an environment with [ArgoCD](https://argo-cd.readthedocs.io/en/stable/) or [Fleet](https://fleet.rancher.io/).

## Create a Helm umbrella chart

Begin by creating a Helm [umbrella chart](https://helm.sh/docs/howto/charts\_tips\_and\_tricks/#complex-charts-with-many-dependencies). The Helm starter chart has unneeded example resources and values that we remove:

```bash
helm create example-job
rm -rf example-job/templates/*.yaml
rm -rf example-job/templates/tests
echo > example-job/values.yaml
```

Your directory structure should look like this:

```
example-job
├── Chart.yaml
├── charts
├── templates
│   ├── NOTES.txt
│   └── _helpers.tpl
└── values.yaml
```

## Populating Databricks resources

We are now going to create our resources in the `templates/` directory.

### 1. Operator configmap and Databricks access token

{% hint style="info" %}
In a production environment, the Databricks API URL and access token can be sourced via [External Secrets Operator](https://external-secrets.io) in combination with (e.g. AWS Secrets Manager).
{% endhint %}

Create a secret containing your Databricks API URL and a valid access token. The snippet below is for your convenience, to run against the cluster for this example. **Do not create a template and check in your token.**

{% code lineNumbers="true" %}
```bash
cat <<EOF | kubectl apply -f -
apiVersion: v1
kind: Secret
type: Opaque
metadata:
  name: databricks-api-secret
data:
  access_token: $(echo -n 'shhhh' | base64)
  databricks_url: $(echo -n 'https://my-tenant.cloud.databricks.com' | base64)
EOF
```
{% endcode %}

Create the file below. The operator configmap expects a secret name from which to pull its REST configuration.

{% code title="templates/databricks-kube-operator-configmap.yaml" %}
```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: databricks-kube-operator
  namespace: {{ .Release.Namespace }}
data:
  api_secret_name: databricks-api-secret
```
{% endcode %}

### 2. Git Repo and Git Credentials

Public repositories do not require [Git credentials](https://docs.databricks.com/repos/repos-setup.html#add-git-credentials-to-databricks), so here is another "quick snippet" for making the required secret if deploying your own job from a private repo. **As previously mentioned, do not check this in as a template.**

```
```
