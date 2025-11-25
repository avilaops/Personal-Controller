# 🚀 Personal Controller - Deployment Guide

Complete guide for deploying Personal Controller to production.

## 🎯 Deployment Options

### Option 1: Docker Compose (Recommended)
### Option 2: Kubernetes
### Option 3: Bare Metal / VM

## 🐳 Docker Deployment

### 1. Build Images

```bash
# Build API
cd pc-api
docker build -t personal-controller-api:latest .

# Build Web
cd pc-web
docker build -t personal-controller-web:latest .
```

### 2. Docker Compose

```yaml
# docker-compose.yml
version: '3.8'

services:
  aviladb:
    image: avilaops/aviladb:latest
    ports:
      - "8000:8000"
    volumes:
      - aviladb-data:/data
    environment:
      - AVILADB_AUTH=true
      - AVILADB_PASSWORD=${AVILADB_PASSWORD}
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8000/health"]
      interval: 30s
      timeout: 10s
      retries: 3

  api:
    image: personal-controller-api:latest
    ports:
      - "3000:3000"
    depends_on:
      - aviladb
    environment:
      - DATABASE_URL=http://aviladb:8000
      - RUST_LOG=info
      - LLM_MODEL=local
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:3000/health"]
      interval: 30s
      timeout: 10s
      retries: 3

  web:
    image: personal-controller-web:latest
    ports:
      - "80:3001"
    depends_on:
      - api
    environment:
      - NEXT_PUBLIC_API_URL=http://api:3000

volumes:
  aviladb-data:
```

### 3. Deploy

```bash
docker-compose up -d
```

## ☸️ Kubernetes Deployment

### 1. Create Namespace

```yaml
# namespace.yaml
apiVersion: v1
kind: Namespace
metadata:
  name: personal-controller
```

### 2. AvilaDB Deployment

```yaml
# aviladb-deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: aviladb
  namespace: personal-controller
spec:
  replicas: 3
  selector:
    matchLabels:
      app: aviladb
  template:
    metadata:
      labels:
        app: aviladb
    spec:
      containers:
      - name: aviladb
        image: avilaops/aviladb:latest
        ports:
        - containerPort: 8000
        env:
        - name: AVILADB_PASSWORD
          valueFrom:
            secretKeyRef:
              name: aviladb-secret
              key: password
        volumeMounts:
        - name: data
          mountPath: /data
      volumes:
      - name: data
        persistentVolumeClaim:
          claimName: aviladb-pvc
---
apiVersion: v1
kind: Service
metadata:
  name: aviladb
  namespace: personal-controller
spec:
  selector:
    app: aviladb
  ports:
  - port: 8000
    targetPort: 8000
```

### 3. API Deployment

```yaml
# api-deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: api
  namespace: personal-controller
spec:
  replicas: 3
  selector:
    matchLabels:
      app: api
  template:
    metadata:
      labels:
        app: api
    spec:
      containers:
      - name: api
        image: personal-controller-api:latest
        ports:
        - containerPort: 3000
        env:
        - name: DATABASE_URL
          value: "http://aviladb:8000"
        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
          limits:
            memory: "512Mi"
            cpu: "500m"
---
apiVersion: v1
kind: Service
metadata:
  name: api
  namespace: personal-controller
spec:
  type: LoadBalancer
  selector:
    app: api
  ports:
  - port: 80
    targetPort: 3000
```

### 4. Deploy to K8s

```bash
kubectl apply -f namespace.yaml
kubectl apply -f aviladb-deployment.yaml
kubectl apply -f api-deployment.yaml
kubectl apply -f web-deployment.yaml
```

## 🔐 Security

### Environment Variables

```bash
# .env.production
DATABASE_URL=https://aviladb.avila.cloud:8000
AVILADB_PASSWORD=<strong-password>
JWT_SECRET=<random-secret>
CORS_ORIGIN=https://controller.avila.cloud
LLM_API_KEY=<api-key>
```

### SSL/TLS

```nginx
# nginx.conf
server {
    listen 443 ssl http2;
    server_name controller.avila.cloud;

    ssl_certificate /etc/ssl/certs/controller.crt;
    ssl_certificate_key /etc/ssl/private/controller.key;

    location / {
        proxy_pass http://localhost:3001;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }

    location /api {
        proxy_pass http://localhost:3000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

## 📊 Monitoring

### Prometheus Metrics

```yaml
# prometheus.yml
scrape_configs:
  - job_name: 'personal-controller-api'
    static_configs:
      - targets: ['api:3000']

  - job_name: 'aviladb'
    static_configs:
      - targets: ['aviladb:8000']
```

### Grafana Dashboards

```bash
# Import dashboards
grafana-cli plugins install grafana-piechart-panel
grafana-cli plugins install grafana-worldmap-panel
```

## 🔄 CI/CD Pipeline

### GitHub Actions

```yaml
# .github/workflows/deploy.yml
name: Deploy

on:
  push:
    branches: [ main ]

jobs:
  build-and-deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2

      - name: Build API
        run: |
          cd pc-api
          cargo build --release

      - name: Build Docker Images
        run: |
          docker build -t personal-controller-api:${{ github.sha }} pc-api
          docker build -t personal-controller-web:${{ github.sha }} pc-web

      - name: Push to Registry
        run: |
          docker push personal-controller-api:${{ github.sha }}
          docker push personal-controller-web:${{ github.sha }}

      - name: Deploy to K8s
        run: |
          kubectl set image deployment/api api=personal-controller-api:${{ github.sha }}
```

## 📈 Scaling

### Horizontal Scaling

```bash
# Scale API replicas
kubectl scale deployment api --replicas=5 -n personal-controller

# Auto-scaling
kubectl autoscale deployment api --cpu-percent=70 --min=3 --max=10
```

### Vertical Scaling

```yaml
resources:
  requests:
    memory: "512Mi"
    cpu: "500m"
  limits:
    memory: "2Gi"
    cpu: "2000m"
```

## 🔧 Maintenance

### Backup

```bash
# Backup AvilaDB
kubectl exec -it aviladb-0 -n personal-controller -- aviladb backup --output /backup

# Copy to local
kubectl cp personal-controller/aviladb-0:/backup ./backup
```

### Updates

```bash
# Rolling update
kubectl set image deployment/api api=personal-controller-api:v2.0.0

# Rollback if needed
kubectl rollout undo deployment/api
```

## 📊 Health Checks

```bash
# API health
curl https://controller.avila.cloud/health

# AvilaDB health
curl https://aviladb.avila.cloud/health

# Web health
curl https://controller.avila.cloud/
```

## 🚨 Troubleshooting

### Check Logs

```bash
# API logs
kubectl logs -f deployment/api -n personal-controller

# AvilaDB logs
kubectl logs -f deployment/aviladb -n personal-controller
```

### Debug Pod

```bash
kubectl run debug --image=busybox -it --rm -n personal-controller
```

---

**Deploy with confidence! 🚀**
