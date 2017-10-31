all:
	docker build -t rust .
	docker run -ti -v $(PWD):/opt rust
