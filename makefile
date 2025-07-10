build-container:
	bash cicd/build.sh build

run-container:
	bash cicd/build.sh run

serve-local:
	dx serve

clean:
	bash cicd/build.sh stop
	docker system prune -a --volumes

