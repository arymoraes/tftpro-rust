import dotenv from 'dotenv';
import path from 'path';
import { createConnection } from 'typeorm';
import { Character } from './entities/Character';

dotenv.config({
  path: path.resolve(__dirname, '../.env'),
});

const databaseConnection = async () => {
  try {
    await createConnection({
      entities: [Character],
      port: 5432,
      synchronize: true, // DO NOT USE FOR PRODUCTION
      type: 'postgres',
      url: process.env.DATABASE_URL,
    });
  } catch (err) {
    console.log(err);
  }
};

export default databaseConnection;
