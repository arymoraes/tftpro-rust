import dotenv from 'dotenv';
import path from 'path';
import { createConnection } from 'typeorm';
import { Character } from './entities/Character';
import { Item } from './entities/Item';
import { Trait } from './entities/Trait';

dotenv.config({
  path: path.resolve(__dirname, '../.env'),
});

const databaseConnection = async () => {
  try {
    await createConnection({
      entities: [Character, Item, Trait],
      port: 5432,
      type: 'postgres',
      url: process.env.DATABASE_URL,
    });
  } catch (err) {
    console.log(err);
  }
};

export default databaseConnection;
