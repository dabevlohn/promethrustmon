alias t := gettext
alias p := plot
url := 'http://localhost:9090/prometheus-metrics'

# All recipes
dummy:
	just -l

# Get HEAD requests text
gettext:
	curl -s '{{url}}'

# Get HEAD requests hits and plot
plot:
	while true; do curl -s "{{url}}" | grep ^tokio_workers | awk '{print $2}'; sleep 1; done | ttyplot -t "Tokio Metrics" -u Workers -m 10 -e ^ -M 1 -E _
