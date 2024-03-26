FROM uhstray/demo_dashboard:latest
WORKDIR /demo_dashboard
COPY . .
RUN dx serve --platform fullstack --port 8080