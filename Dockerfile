FROM nginx:stable-alpine

COPY ./ops/nginx.conf /etc/nginx/conf.d/default.conf
ADD ./content /home/content

# environment
ENV PORT 8080
