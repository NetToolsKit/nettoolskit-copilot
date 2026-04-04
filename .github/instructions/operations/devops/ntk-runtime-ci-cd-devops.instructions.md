---
applyTo: "**/{pipeline,ci,cd,deploy,build,workflow,action}*/**/*.{yml,yaml,json,ps1,sh,dockerfile}"
priority: medium
---

# CI/CD And DevOps Platform

Use this instruction for pipeline architecture, stage boundaries, promotion
flows, artifact handling, and delivery strategy across CI/CD systems. Use
`ntk-runtime-workflow-generation.instructions.md` for GitHub Actions-specific
workflow authoring requirements, and use
`ntk-security-cicd-supply-chain-hardening.instructions.md` for trusted
workflow boundaries, action pinning, OIDC, runner isolation, and provenance
controls.

## Pipeline Structure

- Separate build, test, package, validation, and deploy concerns into explicit stages or jobs.
- Use fail-fast behavior where early failure avoids wasted compute.
- Keep environment promotion explicit instead of hiding it behind one monolithic job.
- Make rollback or recovery paths clear when deployment stages mutate live systems.
- Keep infrastructure-as-code and application delivery coordinated but not conflated.

## CI Baseline

- Run on pull requests and protected branches where appropriate.
- Keep build, test, quality, and security signals explicit.
- Use matrix execution only when real compatibility coverage is needed.
- Keep coverage, lint, and static analysis visible in pipeline outputs.
- Prefer deterministic commands over opaque wrapper chains.

## Build Optimization

- Cache dependency stores intentionally.
- Reuse immutable artifacts between later stages when safe.
- Prefer incremental work only when it does not compromise determinism.
- Keep build-time monitoring and artifact retention visible to operators.
- Avoid pipelines that rebuild the same deliverable repeatedly without reason.

## Testing And Release Gates

- Unit, integration, E2E, performance, and migration tests should run only where they add real release confidence.
- Keep test pyramid responsibilities explicit.
- Coordinate deployment gates with health checks, smoke checks, and rollback conditions.
- Use feature flags or staged rollout controls when delivery risk is material.

## Environments And Promotion

- Treat environment parity as an explicit concern.
- Keep configuration, secrets, and network boundaries environment-specific and reviewable.
- Use promotion gates, approvals, or deployment strategies intentionally.
- Prefer deployable artifacts that stay immutable across environments.
- Coordinate schema or migration changes with deployment order.

## Platform Guardrails

- Keep permissions least-privilege.
- Keep secrets out of logs and summaries.
- Prefer stable artifact names and locations.
- Avoid hidden side effects such as PR mutation or repository rewrites unless explicitly requested.
- Document the system-specific workflow constraints close to the implementation surface.
- canary
- feature flags
- health checks
- monitoring alerts
- automatic rollback
- DB migration coordination
```yaml
stages:
  - stage: DeployStaging
    jobs:
      - deployment: DeployStaging
        environment: 'staging'
        strategy:
          blueGreen:
            deploy:
              steps:
                - task: AzureRmWebAppDeployment@4
                  inputs:
                    ConnectionType: 'AzureRM'
                    azureSubscription: 'subscription'
                    appType: 'webApp'
                    WebAppName: 'myapp-staging'
                    packageForLinux: '$(Pipeline.Workspace)/drop/**/*.zip'
            routeTraffic:
              steps:
                - task: AzureAppServiceManage@0
                  inputs:
                    azureSubscription: 'subscription'
                    Action: 'Swap Slots'
                    WebAppName: 'myapp-staging'
                    ResourceGroupName: 'myrg'
                    SourceSlot: 'staging'
  - stage: DeployProduction
    dependsOn: DeployStaging
    jobs:
      - deployment: DeployProduction
        environment: 'production'
        strategy:
          canary:
            increments: [10, 50]
            preDeploy:
              steps:
                - task: AzureRmWebAppDeployment@4
                  inputs:
                    ConnectionType: 'AzureRM'
                    azureSubscription: 'subscription'
                    appType: 'webApp'
                    WebAppName: 'myapp'
                    packageForLinux: '$(Pipeline.Workspace)/drop/**/*.zip'
            deploy:
              steps:
                - script: echo 'Health check passed'
```

# Environments
- IaC (Terraform/ARM)
- environment parity
- config management
- secrets management
- network isolation
- resource tagging
```hcl
resource "azurerm_resource_group" "example" {
  name     = "rg-example"
  location = "West Europe"
  tags = {
    cost-center = "platform"
  }
}

resource "azurerm_app_service_plan" "example" {
  name                = "asp-example"
  location            = azurerm_resource_group.example.location
  resource_group_name = azurerm_resource_group.example.name
  kind                = "Linux"
  reserved            = true
  sku {
    tier = "Standard"
    size = "S1"
  }
}

resource "azurerm_app_service" "example" {
  name                = "app-example"
  location            = azurerm_resource_group.example.location
  resource_group_name = azurerm_resource_group.example.name
  app_service_plan_id = azurerm_app_service_plan.example.id
  site_config {
    linux_fx_version = "DOTNETCORE|8.0"
  }
}
```

# Kubernetes
- Manifests/helm
- resource limits
- probes
- service discovery
- ingress
- PVs
- security contexts
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: myapp
spec:
  replicas: 3
  selector:
    matchLabels:
      app: myapp
  template:
    metadata:
      labels:
        app: myapp
    spec:
      containers:
      - name: myapp
        image: myregistry/myapp:latest
        ports:
        - containerPort: 80
        resources:
          limits:
            cpu: 500m
            memory: 512Mi
          requests:
            cpu: 250m
            memory: 256Mi
        livenessProbe:
          httpGet:
            path: /health/live
            port: 80
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /health/ready
            port: 80
          initialDelaySeconds: 5
          periodSeconds: 5
        securityContext:
          runAsUser: 1001
          runAsGroup: 1001
      serviceAccountName: myapp-sa
---
apiVersion: v1
kind: Service
metadata:
  name: myapp-service
spec:
  selector:
    app: myapp
  ports:
    - protocol: TCP
      port: 80
      targetPort: 80
  type: ClusterIP
---
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: myapp-ingress
spec:
  tls:
  - hosts:
    - myapp.example.com
    secretName: myapp-tls
  rules:
  - host: myapp.example.com
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: myapp-service
            port:
              number: 80
```

# Monitoring
- App metrics
- infra monitoring
- distributed tracing
- log aggregation
- alerting
- SLAs
- error tracking
```yaml
apiVersion: monitoring.coreos.com/v1
kind: ServiceMonitor
metadata:
  name: myapp-monitor
spec:
  selector:
    matchLabels:
      app: myapp
  endpoints:
  - port: metrics
    path: /metrics
---
apiVersion: monitoring.coreos.com/v1
kind: PrometheusRule
metadata:
  name: myapp-alerts
spec:
  groups:
  - name: myapp
    rules:
    - alert: HighErrorRate
      expr: rate(http_requests_total{status=~"5.."}[5m]) / rate(http_requests_total[5m]) > 0.05
      for: 5m
      labels:
        severity: critical
      annotations:
        summary: High error rate detected
```

# Security
- Least privilege
- image scanning
- dependency auditing
- secret rotation
- network policies
- TLS everywhere
- audit logging
- compliance
```yaml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: deny-all
spec:
  podSelector: {}
  policyTypes:
  - Ingress
  - Egress
---
apiVersion: v1
kind: Secret
metadata:
  name: myapp-secret
type: Opaque
data:
  password: <base64-encoded>
---
apiVersion: apps/v1
kind: Deployment
metadata:
  name: myapp
spec:
  template:
    spec:
      containers:
      - name: myapp
        image: myregistry/myapp:latest
        securityContext:
          runAsUser: 1001
          runAsGroup: 1001
```

# GitOps
- Git-based deploys
- declarative configs
- automated sync
- drift detection
- access control
- approvals
- audit trails
- DR
```yaml
apiVersion: argoproj.io/v1alpha1
kind: Application
metadata:
  name: myapp
spec:
  project: default
  source:
    repoURL: https://github.com/myorg/myrepo
    targetRevision: HEAD
    path: k8s
  destination:
    server: https://kubernetes.default.svc
    namespace: default
  syncPolicy:
    automated:
      prune: true
      selfHeal: true
```

# Performance
- Requests/limits
- HPA
- cluster autoscaling
- CDN
- caching
- DB optimization
- connection pooling
```yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: myapp-hpa
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: myapp
  minReplicas: 1
  maxReplicas: 10
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
---
apiVersion: apps/v1
kind: Deployment
metadata:
  name: myapp
spec:
  template:
    spec:
      containers:
      - name: myapp
        resources:
          limits:
            cpu: 500m
            memory: 512Mi
          requests:
            cpu: 250m
            memory: 256Mi
```

# Backups
- Automated
- cross-region replication
- verification
- restore testing
- DR procedures
- RTO/RPO
- retention policies
```bash
#!/bin/bash
# Backup script
DATE=$(date +%Y%m%d)
BACKUP_DIR="/backups"
DB_NAME="mydb"
pg_dump $DB_NAME > $BACKUP_DIR/$DB_NAME-$DATE.sql
aws s3 cp $BACKUP_DIR/$DB_NAME-$DATE.sql s3://my-backup-bucket/
```

# Observability Stack
- Structured logs
- metrics
- tracing
- error tracking
- performance
- business metrics
- cost
- capacity planning
```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: fluent-bit-config
data:
  fluent-bit.conf: |
    [SERVICE]
        Flush         5
        Log_Level     info
        Daemon        off
    [INPUT]
        Name              tail
        Path              /var/log/containers/*.log
        Parser            docker
        Tag               kube.*
        Refresh_Interval  5
    [OUTPUT]
        Name  es
        Match kube.*
        Host  elasticsearch
        Port  9200
        Index myapp-logs
        Type  logs
```