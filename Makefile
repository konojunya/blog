clean:
	rm -rf content
	mkdir content
	touch content/.gitkeep

build:
	docker build . -t nginx-server

serve:
	docker run -p 8080:8080 nginx-server
