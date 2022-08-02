
FROM node:16.16.0-bullseye

WORKDIR /work/wasm
COPY ./wasm/pkg ./pkg


WORKDIR /work/www

COPY ./www/package*.json ./
RUN npm install
COPY ./www/* ./
EXPOSE 8080

CMD npm run build


