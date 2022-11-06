---
description: An example GitOps recipe
---

# ⌨ Tutorial

This repository contains a [PySpark example job](https://github.com/mach-kernel/databricks-kube-operator/blob/master/examples/job.py) (how did you guess it was word count?) that we are going to operationalize using Helm and databricks-kube-operator. You can follow along with a local [minikube](https://minikube.sigs.k8s.io/docs/) cluster, or use in an environment with [ArgoCD](https://argo-cd.readthedocs.io/en/stable/) or [Fleet](https://fleet.rancher.io/).

## Create a Helm umbrella chart

Begin by creating a Helm [umbrella chart](https://helm.sh/docs/howto/charts\_tips\_and\_tricks/#complex-charts-with-many-dependencies). The Helm starter chart has unneeded example resources and values that we remove:

```bash
helm create example-job
rm example-job/templates/NOTES.txt
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

In `Chart.yaml`, add a dependency to the operator chart:

```yaml
dependencies:
  - name: databricks-kube-operator
    repository: https://mach-kernel.github.io/databricks-kube-operator
    version: 0.1.0
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

### 2. Git Credentials

Public repositories do not require [Git credentials](https://docs.databricks.com/repos/repos-setup.html#add-git-credentials-to-databricks). The tutorial deploys the job from this public repository. You can skip this step, unless you are following along with your own job and a private repo.

Here is another "quick snippet" for making the required secret if deploying your own job from a private repo. **As previously mentioned, do not check this in as a template.**

{% code lineNumbers="true" %}
```bash
cat <<EOF | kubectl apply -f -
apiVersion: v1
kind: Secret
type: Opaque
metadata:
  name: my-git-credential
data:
  access_token: $(echo -n 'shhhh' | base64)
  databricks_url: $(echo -n 'https://my-tenant.cloud.databricks.com' | base64)
EOF
```
{% endcode %}

Create the file below. According to the [API documentation](https://docs.databricks.com/dev-tools/api/latest/gitcredentials.html), the following VCS providers are available:

> The available Git providers are awsCodeCommit, azureDevOpsServices, bitbucketCloud, bitbucketServer, gitHub, gitHubEnterprise, gitLab, and gitLabEnterpriseEdition.

<pre class="language-yaml" data-title="template/git-credential.yaml" data-line-numbers><code class="lang-yaml">apiVersion: com.dstancu.databricks/v1
kind: GitCredential
<strong>metadata:
</strong>  annotations:
    databricks-operator/owner: operator
  name: example-credential
  namespace: {{ .Release.Namespace }}
spec:
  secret_name: my-git-credential
  credential:
    git_username: my-user-name
    git_provider: gitHub</code></pre>

### 3. DatabricksJob

Create the file below to create a job. There are two possible strategies for running jobs via Git sources. For more possible configuration, see the [API SDK docs](databricks-rust-jobs/docs/JobsCreateRequest.md).

#### Using the Git provider

If your credentials are configured, Databricks job definitions [now support](https://docs.databricks.com/repos/ci-cd-best-practices-with-repos.html#run-jobs-using-a-notebook-in-a-databricks-repo) directly referencing a Git source. Whenever the job is triggered, it will use the latest version from source control without needing to poll the repo for updates.&#x20;

{% code title="template/my-word-count.yaml" lineNumbers="true" %}
```yaml
apiVersion: com.dstancu.databricks/v1
kind: DatabricksJob
metadata:
  name: my-word-count
  namespace: {{ .Release.Namespace }}
spec:
  job:
    settings:
      email_notifications:
        no_alert_for_skipped_runs: false
      format: MULTI_TASK
      job_clusters:
      - job_cluster_key: word-count-cluster
        new_cluster:
          aws_attributes:
            availability: SPOT_WITH_FALLBACK
            ebs_volume_count: 1
            ebs_volume_size: 32
            ebs_volume_type: GENERAL_PURPOSE_SSD
            first_on_demand: 1
            spot_bid_price_percent: 100
            zone_id: us-east-1e
          custom_tags:
            ResourceClass: SingleNode
          driver_node_type_id: m4.large
          enable_elastic_disk: false
          node_type_id: m4.large
          num_workers: 0
          spark_conf:
            spark.databricks.cluster.profile: singleNode
            spark.master: local[*, 4]
          spark_env_vars:
            PYSPARK_PYTHON: /databricks/python3/bin/python3
          spark_version: 10.4.x-scala2.12
      max_concurrent_runs: 1
      name: my-word-count
      git_source:
        git_branch: misc-and-docs
        git_provider: gitHub
        git_url: https://github.com/mach-kernel/databricks-kube-operator
      tasks:
      - email_notifications: {}
        job_cluster_key: word-count-cluster
        notebook_task:
          notebook_path: examples/job.py
          source: GIT
        task_key: my-word-count
        timeout_seconds: 0
      timeout_seconds: 0
```
{% endcode %}

#### Using the repos / workspace integration

Follow the [optional Git Repo instructions](tutorial.md#optional-git-repo) before proceeding.&#x20;

This is for use with the `Repo` API, which clones a repository to your workspace. Tasks are then launched from `WORKSPACE` paths. You can reuse the CRD from above removing `git_source` and changing the task definition to match the example below:

{% code title="templates/my-word-count-job.yaml" lineNumbers="true" %}
```yaml
apiVersion: com.dstancu.databricks/v1
kind: DatabricksJob
metadata:
  name: my-word-count
  namespace: {{ .Release.Namespace }}
spec:
  job:
    settings:
      tasks:
      - email_notifications: {}
        job_cluster_key: word-count-cluster
        notebook_task:
          notebook_path: /Repos/Test/databricks-kube-operator/examples/job
          source: WORKSPACE
        task_key: my-word-count
        timeout_seconds: 0
```
{% endcode %}

### 4. All together now

Awesome! We have templates for our shiny new job. Let's make sure the chart works as expected. Inspect the resulting templates for errors:

```
helm template example-job
```

If everything looks good, it's time to install. Unfortunately this requires discussion of the [dreaded "install CRDs first"](https://helm.sh/docs/chart\_best\_practices/custom\_resource\_definitions/#install-a-crd-declaration-before-using-the-resource) problem. Here are suggestions for different readers:

* Local/minikube: Comment out the dependency key and continue with installation
* ArgoCD: Use [sync waves](https://argo-cd.readthedocs.io/en/stable/user-guide/sync-waves/)
* Fleet/others: Use one chart for your operator deployment, and another for the Databricks resources. On first deploy, the operator chart will sync successfully and `example-job` will do so on retry.

```
helm install word-count example-job
```

If successful, you should see the following Helm deployments, as well as your job in Databricks:

```
NAME                            NAMESPACE       REVISION        UPDATED                                 STATUS          CHART                           APP VERSION
databricks-kube-operator        default         1               2022-11-06 09:54:53.057226 -0500 EST    deployed        databricks-kube-operator-0.1.0  1.16.0
word-count                      default         1               2022-11-06 10:11:42.774865 -0500 EST    deployed        example-job-0.1.0               1.16.0
```

Bump the chart version for your Databricks definitions as they change, and let databricks-kube-operator reconcile them when they are merged to your main branch.

### Optional: Git Repo

We recommend using the Git source for your job definitions, as databricks-kube-operator **does not** poll Databricks to update the workspace repository clone. PRs are accepted.

Create the file below to create a repo. Ensure that the `/Test` directory exists within Repos [(docs)](https://docs.databricks.com/repos/work-with-notebooks-other-files.html) on your Databricks instance, or else the create request will 400:

{% code title="templates/repo.yaml" lineNumbers="true" %}
```yaml
apiVersion: com.dstancu.databricks/v1
kind: Repo
metadata:
  annotations:
    databricks-operator/owner: operator
  name: databricks-kube-operator
  namespace: {{ .Release.Namespace }}
spec:
  repository:
    path: /Repos/Test/databricks-kube-operator
    provider: gitHub
    url: https://github.com/mach-kernel/databricks-kube-operator.git
```
{% endcode %}
