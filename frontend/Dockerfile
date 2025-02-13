FROM node AS dev

WORKDIR /app

ENV NEXT_TELEMETRY_DISABLED 1

COPY . .
RUN npm i

CMD ["npm", "run", "dev"]
