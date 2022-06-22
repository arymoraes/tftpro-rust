import { Character } from '../entities/Character';
import databaseConnection from '../index';
import charactersJson from './assets/characters.json';

const characters = [
  'https://rerollcdn.com/characters/Skin/7/AoShin.png',
  'https://rerollcdn.com/characters/Skin/7/Bard.png',
  'https://rerollcdn.com/characters/Skin/7/Diana.png',
  'https://rerollcdn.com/characters/Skin/7/Olaf.png',
  'https://rerollcdn.com/characters/Skin/7/Ornn.png',
  'https://rerollcdn.com/characters/Skin/7/Talon.png',
  'https://rerollcdn.com/characters/Skin/7/Yasuo.png',
  'https://rerollcdn.com/characters/Skin/7/AurelionSol.png',
  'https://rerollcdn.com/characters/Skin/7/Daeja.png',
  'https://rerollcdn.com/characters/Skin/7/Ezreal.png',
  'https://rerollcdn.com/characters/Skin/7/Hecarim.png',
  'https://rerollcdn.com/characters/Skin/7/Idas.png',
  'https://rerollcdn.com/characters/Skin/7/Lulu.png',
  'https://rerollcdn.com/characters/Skin/7/Pyke.png',
  'https://rerollcdn.com/characters/Skin/7/Qiyana.png',
  'https://rerollcdn.com/characters/Skin/7/Shen.png',
  'https://rerollcdn.com/characters/Skin/7/ShiOhYu.png',
  'https://rerollcdn.com/characters/Skin/7/Shyvana.png',
  'https://rerollcdn.com/characters/Skin/7/Soraka.png',
  'https://rerollcdn.com/characters/Skin/7/Syfen.png',
  'https://rerollcdn.com/characters/Skin/7/Sylas.png',
  'https://rerollcdn.com/characters/Skin/7/Xayah.png',
  'https://rerollcdn.com/characters/Skin/7/Zoe.png',
  'https://rerollcdn.com/characters/Skin/7/Ashe.png',
  'https://rerollcdn.com/characters/Skin/7/Braum.png',
  'https://rerollcdn.com/characters/Skin/7/Corki.png',
  'https://rerollcdn.com/characters/Skin/7/Elise.png',
  'https://rerollcdn.com/characters/Skin/7/Gnar.png',
  'https://rerollcdn.com/characters/Skin/7/Heimerdinger.png',
  'https://rerollcdn.com/characters/Skin/7/Illaoi.png',
  'https://rerollcdn.com/characters/Skin/7/Jinx.png',
  'https://rerollcdn.com/characters/Skin/7/Kayn.png',
  'https://rerollcdn.com/characters/Skin/7/LeeSin.png',
  'https://rerollcdn.com/characters/Skin/7/Leona.png',
  'https://rerollcdn.com/characters/Skin/7/Nami.png',
  'https://rerollcdn.com/characters/Skin/7/Neeko.png',
  'https://rerollcdn.com/characters/Skin/7/Nidalee.png',
  'https://rerollcdn.com/characters/Skin/7/Nunu.png',
  'https://rerollcdn.com/characters/Skin/7/Ryze.png',
  'https://rerollcdn.com/characters/Skin/7/Sett.png',
  'https://rerollcdn.com/characters/Skin/7/Sona.png',
  'https://rerollcdn.com/characters/Skin/7/Swain.png',
  'https://rerollcdn.com/characters/Skin/7/Taric.png',
  'https://rerollcdn.com/characters/Skin/7/Tristana.png',
  'https://rerollcdn.com/characters/Skin/7/Twitch.png',
  'https://rerollcdn.com/characters/Skin/7/Varus.png',
  'https://rerollcdn.com/characters/Skin/7/Vladimir.png',
  'https://rerollcdn.com/characters/Skin/7/Yone.png',
  'https://rerollcdn.com/characters/Skin/7/Aatrox.png',
  'https://rerollcdn.com/characters/Skin/7/Anivia.png',
  'https://rerollcdn.com/characters/Skin/7/Karma.png',
  'https://rerollcdn.com/characters/Skin/7/Lillia.png',
  'https://rerollcdn.com/characters/Skin/7/Senna.png',
  'https://rerollcdn.com/characters/Skin/7/Thresh.png',
  'https://rerollcdn.com/characters/Skin/7/Volibear.png',
  'https://rerollcdn.com/characters/Skin/7/Sejuani.png',
  'https://rerollcdn.com/characters/Skin/7/Skarner.png',
  'https://rerollcdn.com/characters/Skin/7/TahmKench.png',
];

interface CharacterInterface {
  character_id: string;
  display_name: string;
  path: string;
  rarity: number;
  squareIconPath: string;
}

const insertCharacter = async (char: CharacterInterface) => {
  const charIndex = characters.findIndex((charString) =>
    charString.includes(char.display_name)
  );

  await Character.create({
    ...char,
    img: charIndex ? characters[charIndex] : null,
    square_icon_path: char.squareIconPath,
  }).save();
};

databaseConnection().then(() => {
  charactersJson.forEach((char) => insertCharacter(char));
});
