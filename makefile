build-container:
	bash cicd/build.sh build

run-container:
	bash cicd/build.sh run

serve-local:
	dx serve

clean:
	docker system prune -a --volumes
