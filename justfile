alias t := gettext
alias s := spawn
alias p := plot
alias r := start_prometheus
purl := 'http://localhost:9091/metrics'
surl := 'http://localhost:9091/spawn-thread'

# All recipes
dummy:
	just -l

# Start Prometheus in Podman
start_prometheus:
    podman run -d -p 9090:9090 -v ./prometheus.yml:/etc/prometheus/prometheus.yml prom/prometheus

# Get sum of requests in text once
gettext:
	curl -s '{{purl}}'

# Spawn new worker once
spawn:
	curl -s '{{surl}}'

# Load application by GET-requests
wrk:
    wrk -t16 -c100 -d60s '{{purl}}'

# View picks in terminal
plot:
	while true; do curl -s "{{purl}}" | grep ^tokio_workers | awk '{print $2}'; sleep 1; done | ttyplot -t "Tokio Metrics" -u Workers -m 10 -e ^ -M 1 -E _
