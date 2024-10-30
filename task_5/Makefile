TASK_SERVICE_IMAGE_NAME = task_service
PORT = 8080	

run-postgres-redis:
			@docker-compose -f local-docker-compose.yaml up -d

run-task-service:
			@cd task_service && cargo run -- -p local-config.toml -a localhost:8080

run-task-executor:
			@cd task_executor && cargo run -- -p local-config.toml
	
build-task-service:
			@cd task_service && docker build -t $(TASK_SERVICE_IMAGE_NAME) .	 