import { PrismaClient } from '@prisma/client';
import bodyParser from 'body-parser';
import cors from 'cors';
import dotenv from 'dotenv';
import express from 'express';
import router from './router';

dotenv.config();

const app = express();

const corsConfig = {
  credentials: true,
  origin: '*',
};

app.use(express.json());

app.use(cors(corsConfig));
app.use(router);

export const prismaClient = new PrismaClient();

(() => {
  app.listen(process.env.PORT || 4000, (): void => {
    console.log(`Server running on ${process.env.PORT || 4000}`);
  });
})();
