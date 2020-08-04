IMAGE_URL := gcr.io/blog-konojunya-com/blog

clean:
	rm -rf content
	mkdir content
	touch content/.gitkeep

build:
	docker build . -t $(IMAGE_URL)

serve:
	env PORT=8080 docker run -p 8080:8080 nginx-server

push:
	gcloud builds submit --tag $(IMAGE_URL)

deploy:
	gcloud run deploy blog --image $(IMAGE_URL):latest --platform managed
