FROM nginx:alpine

COPY ./ops/nginx.conf /etc/nginx/conf.d/configfile.template
ENV PORT 8080
ENV HOST 0.0.0.0
RUN sh -c "envsubst '\$PORT'  < /etc/nginx/conf.d/configfile.template > /etc/nginx/conf.d/default.conf"
ADD ./content /home/content
EXPOSE 8080
CMD ["nginx", "-g", "daemon off;"]
