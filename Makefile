.PHONY: develop
develop:
	maturin develop


test: develop
    ## TODO: Add actual tests with pytest
	python tests/test.py
