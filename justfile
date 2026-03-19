alias t := gettext
alias s := spawn
alias p := plot
purl := 'http://localhost:9091/prometheus-metrics'
surl := 'http://localhost:9091/spawn-thread'

# All recipes
dummy:
	just -l

# Get HEAD requests text
gettext:
	curl -s '{{purl}}'

spawn:
	curl -s '{{surl}}'

# Get HEAD requests hits and plot
plot:
	while true; do curl -s "{{purl}}" | grep ^tokio_workers | awk '{print $2}'; sleep 1; done | ttyplot -t "Tokio Metrics" -u Workers -m 10 -e ^ -M 1 -E _
