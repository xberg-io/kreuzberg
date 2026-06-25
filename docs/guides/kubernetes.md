# Kubernetes Deployment

Deploy Xberg to Kubernetes with proper OCR configuration, permissions, and health checks.

## Helm Chart

Deploy via the official Helm chart (OCI artifact on GHCR).

### Install

```bash title="Terminal"
helm install xberg oci://ghcr.io/xberg-io/charts/xberg --version 4.8.4
```

### Configure

Override defaults with a `values.yaml` file:

```yaml title="values.yaml"
# NOTE: cache.enabled=true uses ReadWriteOnce by default; keep replicaCount: 1
# with RWO storage or switch to ReadWriteMany before increasing replicas.
replicaCount: 1

image:
  tag: "4.8.4"

xberg:
  logLevel: "info"
  ocrLanguage: "eng"

resources:
  requests:
    memory: "1Gi"
    cpu: "1000m"
  limits:
    memory: "4Gi"
    cpu: "2000m"

cache:
  enabled: true
  size: 5Gi

ingress:
  enabled: true
  className: "nginx"
  hosts:
    - host: xberg.example.com
      paths:
        - path: /
          pathType: Prefix
  tls:
    - secretName: xberg-tls
      hosts:
        - xberg.example.com

autoscaling:
  enabled: true
  minReplicas: 2
  maxReplicas: 10
  targetCPUUtilizationPercentage: 80

podDisruptionBudget:
  enabled: true
  minAvailable: 1
```

```bash title="Terminal"
helm install xberg oci://ghcr.io/xberg-io/charts/xberg \
  --version 4.8.4 \
  -f values.yaml
```

### Upgrade

```bash title="Terminal"
helm upgrade xberg oci://ghcr.io/xberg-io/charts/xberg --version 4.8.4
```

### What's Included

The chart creates the following resources:

| Resource                | Description                                                | Conditional                   |
| ----------------------- | ---------------------------------------------------------- | ----------------------------- |
| Deployment              | Main application with health probes and security hardening | Always                        |
| Service                 | ClusterIP service on port 80 → 8000                        | Always                        |
| ServiceAccount          | Dedicated service account                                  | Always                        |
| PersistentVolumeClaim   | Cache for embedding models and assets                      | `cache.enabled`               |
| Ingress                 | HTTP(S) ingress with TLS                                   | `ingress.enabled`             |
| HorizontalPodAutoscaler | CPU/memory-based autoscaling                               | `autoscaling.enabled`         |
| PodDisruptionBudget     | Availability during disruptions                            | `podDisruptionBudget.enabled` |

All values are documented in the chart's [`values.yaml`](https://github.com/xberg-io/xberg/blob/main/charts/xberg/values.yaml).

---

## Quick Start

```yaml title="minimal-deployment.yaml"
apiVersion: apps/v1
kind: Deployment
metadata:
  name: xberg-api
spec:
  replicas: 2
  selector:
    matchLabels:
      app: xberg
  template:
    metadata:
      labels:
        app: xberg
    spec:
      containers:
        - name: xberg
          image: ghcr.io/xberg-io/xberg:latest
          ports:
            - containerPort: 8000
              name: http
          env:
            - name: RUST_LOG
              value: "info"
            - name: TESSDATA_PREFIX
              value: "/usr/share/tesseract-ocr/5/tessdata"
          resources:
            requests:
              memory: "512Mi"
              cpu: "500m"
            limits:
              memory: "2Gi"
              cpu: "2000m"
          livenessProbe:
            httpGet:
              path: /health
              port: 8000
            initialDelaySeconds: 10
            periodSeconds: 30
          readinessProbe:
            httpGet:
              path: /health
              port: 8000
            initialDelaySeconds: 5
            periodSeconds: 10
---
apiVersion: v1
kind: Service
metadata:
  name: xberg-api
spec:
  selector:
    app: xberg
  ports:
    - protocol: TCP
      port: 80
      targetPort: 8000
  type: LoadBalancer
```

```bash title="Terminal"
kubectl apply -f minimal-deployment.yaml
```

## Tesseract Configuration

### TESSDATA_PREFIX (Critical)

Without `TESSDATA_PREFIX`, OCR silently falls back to non-OCR extraction. Official images ship Tesseract 5.x with tessdata at `/usr/share/tesseract-ocr/5/tessdata/`.

```yaml
env:
  - name: TESSDATA_PREFIX
    value: "/usr/share/tesseract-ocr/5/tessdata"
  - name: XBERG_OCR_LANGUAGE
    value: "eng"
  - name: XBERG_CACHE_DIR
    value: "/app/.xberg"
  - name: HF_HOME
    value: "/app/.xberg/huggingface"
```

**Pre-installed languages:** `eng`, `spa`, `fra`, `deu`, `ita`, `por`, `chi_sim`, `chi_tra`, `jpn`, `ara`, `rus`, `hin`

!!! Note "Tesseract Version" The path varies by version. Verify yours with `tesseract --version` inside the container if using a custom base image.

### Custom Languages via ConfigMap

```bash title="Terminal"
kubectl create configmap tessdata \
  --from-file=/path/to/eng.traineddata \
  --from-file=/path/to/deu.traineddata
```

```yaml
spec:
  containers:
    - name: xberg
      env:
        - name: TESSDATA_PREFIX
          value: "/etc/tessdata"
      volumeMounts:
        - name: tessdata
          mountPath: /etc/tessdata
  volumes:
    - name: tessdata
      configMap:
        name: tessdata
```

For large custom language sets, use a PVC instead of a ConfigMap.

### Verify Tesseract

```bash title="Terminal"
kubectl exec -it deployment/xberg-api -- tesseract --version
kubectl exec -it deployment/xberg-api -- tesseract --list-langs
kubectl exec -it deployment/xberg-api -- printenv TESSDATA_PREFIX
```

## Permissions

Xberg runs as non-root (UID 1000, GID 1000). Fix PVC permissions with either approach:

=== "Init Container"

    ```yaml
    spec:
      initContainers:
      - name: init-permissions
        image: busybox:1.37-glibc
        command: ['sh', '-c', 'chown -R 1000:1000 /app/.xberg']
        securityContext:
          runAsUser: 0
          allowPrivilegeEscalation: false
          capabilities:
            add: ["CHOWN"]
            drop: ["ALL"]
        volumeMounts:
        - name: cache
          mountPath: /app/.xberg
      containers:
      - name: xberg
        volumeMounts:
        - name: cache
          mountPath: /app/.xberg
    ```

=== "fsGroup"

    ```yaml
    spec:
      securityContext:
        fsGroup: 1000
      containers:
      - name: xberg
        securityContext:
          runAsUser: 1000
          runAsGroup: 1000
          allowPrivilegeEscalation: false
          readOnlyRootFilesystem: true
          capabilities:
            drop: ["ALL"]
    ```

## Health Checks

```yaml
containers:
  - name: xberg
    livenessProbe:
      httpGet:
        path: /health
        port: 8000
      initialDelaySeconds: 10
      periodSeconds: 30
      timeoutSeconds: 5
      failureThreshold: 3
    readinessProbe:
      httpGet:
        path: /health
        port: 8000
      initialDelaySeconds: 5
      periodSeconds: 10
      timeoutSeconds: 3
      failureThreshold: 2
    startupProbe:
      httpGet:
        path: /health
        port: 8000
      periodSeconds: 10
      failureThreshold: 30
```

## Logging

```yaml
env:
  - name: RUST_LOG
    value: "xberg=debug,warn"
```

Levels: `trace`, `debug`, `info`, `warn`, `error`

```bash title="Terminal"
kubectl logs deployment/xberg-api --tail=50
kubectl logs deployment/xberg-api -f
kubectl logs deployment/xberg-api --previous
```

## Production Deployment

Full production manifest with namespace, PVC, security context, init container, PDB, and all probes:

```yaml title="production-deployment.yaml"
apiVersion: v1
kind: Namespace
metadata:
  name: xberg
---
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: xberg-cache
  namespace: xberg
spec:
  accessModes: [ReadWriteOnce]
  resources:
    requests:
      storage: 2Gi
---
apiVersion: apps/v1
kind: Deployment
metadata:
  name: xberg-api
  namespace: xberg
  # NOTE: PVC uses ReadWriteOnce; keep replicas: 1 with RWO storage.
  # Increase replicas only when using ReadWriteMany storage.
spec:
  replicas: 1
  selector:
    matchLabels:
      app: xberg
  template:
    metadata:
      labels:
        app: xberg
    spec:
      securityContext:
        runAsNonRoot: true
        runAsUser: 1000
        runAsGroup: 1000
        fsGroup: 1000
        seccompProfile:
          type: RuntimeDefault
      initContainers:
        - name: init-cache
          image: busybox:1.37-glibc
          command: ["sh", "-c", "mkdir -p /app/.xberg && chown -R 1000:1000 /app/.xberg"]
          securityContext:
            runAsUser: 0
            allowPrivilegeEscalation: false
            capabilities:
              add: ["CHOWN"]
              drop: ["ALL"]
          volumeMounts:
            - name: cache
              mountPath: /app/.xberg
      containers:
        - name: xberg
          image: ghcr.io/xberg-io/xberg:latest
          ports:
            - containerPort: 8000
              name: http
          env:
            - name: RUST_LOG
              value: "info"
            - name: TESSDATA_PREFIX
              value: "/usr/share/tesseract-ocr/5/tessdata"
            - name: XBERG_CACHE_DIR
              value: "/app/.xberg"
            - name: HF_HOME
              value: "/app/.xberg/huggingface"
            - name: XBERG_CORS_ORIGINS
              value: "https://app.example.com"
            - name: XBERG_MAX_UPLOAD_SIZE_MB
              value: "500"
          args: ["serve", "--host", "0.0.0.0", "--port", "8000"]
          resources:
            requests:
              memory: "1Gi"
              cpu: "1000m"
            limits:
              memory: "4Gi"
              cpu: "2000m"
          livenessProbe:
            httpGet:
              path: /health
              port: 8000
            initialDelaySeconds: 15
            periodSeconds: 30
          readinessProbe:
            httpGet:
              path: /health
              port: 8000
            initialDelaySeconds: 10
            periodSeconds: 10
          startupProbe:
            httpGet:
              path: /health
              port: 8000
            periodSeconds: 10
            failureThreshold: 30
          securityContext:
            allowPrivilegeEscalation: false
            readOnlyRootFilesystem: true
            capabilities:
              drop: ["ALL"]
          volumeMounts:
            - name: cache
              mountPath: /app/.xberg
            - name: tmp
              mountPath: /tmp
      volumes:
        - name: cache
          persistentVolumeClaim:
            claimName: xberg-cache
        - name: tmp
          emptyDir: {}
---
apiVersion: v1
kind: Service
metadata:
  name: xberg-api
  namespace: xberg
spec:
  type: LoadBalancer
  selector:
    app: xberg
  ports:
    - protocol: TCP
      port: 80
      targetPort: 8000
---
apiVersion: policy/v1
kind: PodDisruptionBudget
metadata:
  name: xberg-pdb
  namespace: xberg
spec:
  minAvailable: 1
  selector:
    matchLabels:
      app: xberg
```

```bash title="Terminal"
kubectl apply -f production-deployment.yaml
```

!!! Note "Model Persistence" Embedding models download on first use (~90 MB – 1.2 GB). Use a PVC for `/app/.xberg` to avoid re-downloading on pod restart. Outside containers, models are cached in the platform-specific global cache directory (for example, `~/.cache/xberg/` on Linux, `~/Library/Caches/xberg/` on macOS).

## High Availability

Add pod anti-affinity and rolling update strategy:

```yaml title="ha-additions.yaml"
spec:
  replicas: 5
  strategy:
    type: RollingUpdate
    rollingUpdate:
      maxSurge: 1
      maxUnavailable: 0
  template:
    spec:
      affinity:
        podAntiAffinity:
          preferredDuringSchedulingIgnoredDuringExecution:
            - weight: 100
              podAffinityTerm:
                labelSelector:
                  matchExpressions:
                    - key: app
                      operator: In
                      values: [xberg]
                topologyKey: kubernetes.io/hostname
```

## Troubleshooting

??? Question "OCR silently failing"

    Verify `TESSDATA_PREFIX` is set and tessdata files exist:

    ```bash title="Terminal"
    kubectl exec -it deployment/xberg-api -- printenv TESSDATA_PREFIX
    kubectl exec -it deployment/xberg-api -- ls /usr/share/tesseract-ocr/5/tessdata/
    ```

??? Question "Permission denied on cache directory"

    Use an init container or `fsGroup` (see [Permissions](#permissions)).

??? Question "OOMKilled"

    Increase memory limits. Reduce OCR resource usage with `XBERG_PDF_DPI=150` and single-language OCR.

??? Question "Startup probe timeout"

    Increase `failureThreshold` on the startup probe (e.g., `60` for 10-minute timeout).

??? Question "Language not found"

    Check installed languages with `kubectl exec -it deployment/xberg-api -- tesseract --list-langs`. Mount custom tessdata via ConfigMap or PVC.

### Diagnostic Commands

```bash title="Terminal"
kubectl logs deployment/xberg-api --tail=200
kubectl describe deployment xberg-api
kubectl get events -n xberg
kubectl exec -it deployment/xberg-api -- env | sort
kubectl port-forward service/xberg-api 8000:8000 && curl http://localhost:8000/health
```

## Next Steps

- [Docker Deployment](docker.md) — container configuration and image variants
- [API Server Guide](api-server.md) — endpoint documentation
- [OCR Guide](ocr.md) — backend installation and language setup
- [Configuration](configuration.md) — all configuration options
