ARG NODE_VERSION=18

FROM node:${NODE_VERSION}-alpine

WORKDIR /app

COPY . .

RUN npm ci --audit false --fund false
RUN npm run build

ENTRYPOINT ["npm", "run", "serve", "--", "--host", "0.0.0.0", "--port", "3001"]

EXPOSE 3001