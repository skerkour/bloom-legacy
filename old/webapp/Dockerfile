# build stage
FROM node:lts-alpine AS builder

RUN apk add make

WORKDIR /app
COPY package*.json ./
RUN npm install
COPY . .
RUN make build

####################################################################################################
## Image
####################################################################################################
FROM nginx:stable-alpine

COPY --from=builder /app/dist /usr/share/nginx/html
EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
