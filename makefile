build-container:
	bash cicd/build.sh build

serve-local:
	dx serve

docker-clean:
	docker prune -a --volumes
