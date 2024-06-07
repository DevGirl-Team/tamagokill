ARG NODE_VERSION=18

# FIRST STAGE : BUILDER

FROM node:${NODE_VERSION}-alpine AS builder
WORKDIR /app
# Copy the files package.json AND package-lock.json
COPY package*.json .
RUN npm ci
COPY . .
RUN npm run build
# This command removes "extraneous" packages. 
# If a package name is provided, then only packages matching one of the supplied names are removed.
RUN npm prune --production

# SECOND STAGE : FINAL

FROM node:${NODE_VERSION}-alpine
WORKDIR /app
COPY --from=builder /app/build build/
COPY --from=builder /app/node_modules node_modules/
COPY package.json .
EXPOSE 3000
ENV NODE_ENV=production
CMD [ "node", "build" ]