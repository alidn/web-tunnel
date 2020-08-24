FROM node:latest AS frontend_builder

WORKDIR /frontend
COPY /static/svelte-frontend/svelte-app .
RUN npm install
RUN npm run build
EXPOSE 5000
CMD ["npm", "run", "start"]

WORKDIR /server

FROM rust:latest

WORKDIR /server
COPY . .
COPY --from=frontend_builder /frontend ./frontend
RUN echo `pwd`
RUN echo $(ls -a)
RUN echo $(ls -a ./frontend)
EXPOSE 8000
RUN cargo build
CMD ["cargo", "run"]

