PROJECT_NAME=hemrs

all: container

build:
	cargo check --verbose
	cargo b --verbose

test: build
	cargo t --verbose

container: test
	docker build -t ghcr.io/frixxie/$(PROJECT_NAME):latest .

publish_container: container
	docker login ghcr.io -u Frixxie --password-stdin $(GITHUB_TOKEN)
	docker push ghcr.io/frixxie/$(PROJECT_NAME):latest

publish_tagged_container: container
	docker tag ghcr.io/frixxie/$(PROJECT_NAME):latest ghcr.io/frixxie/$(PROJECT_NAME):$(TAG)
	docker login ghcr.io -u Frixxie --password-stdin $(GITHUB_TOKEN)
	docker push ghcr.io/frixxie/$(PROJECT_NAME):$(TAG)
