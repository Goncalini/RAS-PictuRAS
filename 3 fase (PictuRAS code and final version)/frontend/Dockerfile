FROM node:22-alpine

WORKDIR /app

COPY package*.json ./

RUN npm install
COPY . .

ARG VITE_API_BASE
ENV VITE_API_BASE=$VITE_API_BASE

RUN npm run build

EXPOSE 3000

CMD ["npm", "run", "preview:dist"]