.PHONY: run ports

FILTER = "label=com.docker.compose.project=1_consistent_hashing"
# given something like 0.0.0.0:8891->8888/tcp, extract the host port: 8891
JQ_PORT_PARSE = (.Ports | split("->")[0] | split(":")[1])
JQ_EXPRESSION = '{id: .ID, ports: $(JQ_PORT_PARSE)}'


run:
	docker-compose up -d
	sleep 2 # Give services time to start
	make ports

ports:
	@echo "Extracting port information..."
	@docker ps --filter $(FILTER) --format  '{{json . }}'| jq $(JQ_EXPRESSION) > ports.json
	@echo "Port information written to ports.json"
