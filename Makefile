clean:
	rm -rf content
	mkdir content
	touch content/.gitkeep

build:
	docker build . -t h2o-server

serve:
	docker run -p 8080:8080 h2o-server
