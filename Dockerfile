FROM docker.io/nginx:alpine

RUN rm -rf /usr/share/nginx/html/*
COPY public/ /usr/share/nginx/html/

ENTRYPOINT ["nginx", "-g", "daemon off;"]
