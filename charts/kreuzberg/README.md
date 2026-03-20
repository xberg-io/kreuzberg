# Kreuzberg Helm Chart

Official Helm chart for deploying [Kreuzberg](https://github.com/kreuzberg-dev/kreuzberg), a document intelligence and OCR service.

## Prerequisites

- Kubernetes 1.19+
- Helm 3.0+
- PV provisioner support in the underlying infrastructure (if persistence is enabled)

## Installing the Chart

To install the chart with the release name `my-release`:

```bash
helm install my-release ./charts/kreuzberg
```

## Configuration

The following table lists the configurable parameters of the Kreuzberg chart and their default values.

| Parameter | Description | Default |
| --------- | ----------- | ------- |
| `replicaCount` | Number of replicas | `1` |
| `image.repository` | Image repository | `kreuzberg-dev/kreuzberg` |
| `image.pullPolicy` | Image pull policy | `IfNotPresent` |
| `image.tag` | Image tag (defaults to `Chart.appVersion`) | `""` |
| `service.type` | Service type | `ClusterIP` |
| `service.port` | Service port | `8000` |
| `ingress.enabled` | Enable ingress | `false` |
| `persistence.enabled` | Enable persistent storage for cache and models | `true` |
| `persistence.size` | Storage size for cache | `10Gi` |
| `resources` | CPU/Memory limits and requests | `{}` |
| `env` | Default environment variables | (see values.yaml) |
| `extraEnv` | Additional environment variables | `[]` |

Refer to [values.yaml](values.yaml) for the full list of configuration options.

## Persistent Storage

Kreuzberg downloads models (PaddleOCR, layout detection, etc.) and caches processed documents. It is highly recommended to enable persistence to avoid re-downloading models on every pod restart.

By default, the chart creates a 10Gi PVC for the cache directory `/app/.kreuzberg`.

## Tesseract Customization

If you need to use custom Tesseract trained data files, you can enable the `tesseract` section and provide a ConfigMap or PVC:

```yaml
tesseract:
  enabled: true
  configMapName: my-custom-tessdata
```

## Health Checks

The chart includes liveness and readiness probes pointing to the `/health` endpoint of the Kreuzberg API.
