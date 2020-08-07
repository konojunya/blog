IMAGE_URL := gcr.io/blog-konojunya-com/blog

build:
	docker build . -t $(IMAGE_URL)

serve:
	env PORT=8080 docker run -p 8080:8080 $(IMAGE_URL)

push:
	gcloud builds submit --tag $(IMAGE_URL)

deploy:
	gcloud run deploy blog --image $(IMAGE_URL):latest --platform managed
