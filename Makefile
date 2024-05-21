# Run all commands in a single shell, so that the virtual environment is activated for all commands.
.ONESHELL:

.PHONY: setup
setup:
	python -m venv .venv
	. .venv/bin/activate
	pip install maturin
	pip install pytest

.PHONY: develop
develop:
	. .venv/bin/activate
	maturin develop

test: develop
	. .venv/bin/activate
	pytest

.PHONY: validate-tags release

VERSION_PY := $(shell grep 'version = ' pyproject.toml | sed -e 's/version = "\(.*\)"/\1/')
VERSION_RS := $(shell grep 'version =' Cargo.toml | sed -n 's/^version = "\(.*\)"/\1/p')

# Usage: make validate-tags TAG=1.0.0
validate-tag:
	@if [ -z "$(TAG)" ]; then \
		echo "Error: No TAG specified. Usage: make validate-tags TAG=1.0.0"; \
		exit 1; \
	fi
	@if [ "$(TAG)" != "$(VERSION_PY)" ]; then \
		echo "Tag $(TAG) does not match version in pyproject.toml $(VERSION_PY)"; \
		exit 1; \
	fi
	@if [ "$(TAG)" != "$(VERSION_RS)" ]; then \
		echo "Tag $(TAG) does not match version in cargo.toml $(VERSION_RS)"; \
		exit 1; \
	fi
	@echo "Tag $(TAG) is valid and matches version in both files."

# Usage: make release TAG=1.0.0
release: validate-tag
	# Check if the current branch is main
	$(eval CURRENT_BRANCH := $(shell git rev-parse --abbrev-ref HEAD))
	@if [ "$(CURRENT_BRANCH)" != "main" ]; then \
		echo "Release can only be performed from the main branch. Current branch is $(CURRENT_BRANCH)."; \
		exit 1; \
	fi
	# Check if we are up to date with the remote
	git fetch
	$(eval LOCAL := $(shell git rev-parse @))
	$(eval REMOTE := $(shell git rev-parse @{u}))
	@if [ "$(LOCAL)" = "$(REMOTE)" ]; then \
		echo "Local branch is up to date with the remote."; \
	else \
		echo "Local branch is not up to date with the remote. Please pull changes before releasing."; \
		exit 1; \
	fi

	@if [ "$(LOCAL)" != "$(REMOTE)" ]; then \
		echo "Local branch is not up to date with the remote. Please pull changes before releasing."; \
		exit 1; \
	fi
	# Check if the latest tag is the same as the current version
	$(eval LATEST_TAG := $(shell git describe --tags --abbrev=0))
	@if [ "$(LATEST_TAG)" = "$(VERSION_PY)" ]; then \
		echo "No version bump detected. Current version $(VERSION_PY) matches the latest tag $(LATEST_TAG)."; \
		exit 1; \
	fi
	git commit -m 'Release $(TAG)' --allow-empty
	git tag $(TAG) -m "$(TAG)"
	git push origin --follow-tags
	@echo "Released new version $(TAG)"
