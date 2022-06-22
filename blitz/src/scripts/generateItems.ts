import { Item } from '../entities/Item';
import databaseConnection from '../index';
import jsonItems from './assets/items.json';

const items = [
  'https://rerollcdn.com/items/RecurveBow.png',
  'https://rerollcdn.com/items/Bloodthirster.png',
  'https://rerollcdn.com/items/GuinsoosRageblade.png',
  'https://rerollcdn.com/items/Quicksilver.png',
  'https://rerollcdn.com/items/SunfireCape.png',
  'https://rerollcdn.com/items/TacticiansCrown.png',
  'https://rerollcdn.com/items/BFSword.png',
  'https://rerollcdn.com/items/SparringGloves.png',
  'https://rerollcdn.com/items/ArchangelsStaff.png',
  'https://rerollcdn.com/items/BrambleVest.png',
  'https://rerollcdn.com/items/DragonsClaw.png',
  'https://rerollcdn.com/items/FrozenHeart.png',
  'https://rerollcdn.com/items/GargoyleStoneplate.png',
  'https://rerollcdn.com/items/GiantSlayer.png',
  'https://rerollcdn.com/items/HandofJustice.png',
  'https://rerollcdn.com/items/HextechGunblade.png',
  'https://rerollcdn.com/items/InfinityEdge.png',
  'https://rerollcdn.com/items/JeweledGauntlet.png',
  'https://rerollcdn.com/items/LastWhisper.png',
  'https://rerollcdn.com/items/Morellonomicon.png',
  'https://rerollcdn.com/items/SpearofShojin.png',
  'https://rerollcdn.com/items/StatikkShiv.png',
  'https://rerollcdn.com/items/ThiefsGloves.png',
  'https://rerollcdn.com/items/TitansResolve.png',
  'https://rerollcdn.com/items/WarmogsArmor.png',
  'https://rerollcdn.com/items/NeedlesslyLargeRod.png',
  'https://rerollcdn.com/items/TearoftheGoddess.png',
  'https://rerollcdn.com/items/BlueBuff.png',
  'https://rerollcdn.com/items/Deathblade.png',
  'https://rerollcdn.com/items/IonicSpark.png',
  'https://rerollcdn.com/items/RabadonsDeathcap.png',
  'https://rerollcdn.com/items/RapidFirecannon.png',
  'https://rerollcdn.com/items/Redemption.png',
  'https://rerollcdn.com/items/RunaansHurricane.png',
  'https://rerollcdn.com/items/Zephyr.png',
  'https://rerollcdn.com/items/ChainVest.png',
  'https://rerollcdn.com/items/GiantsBelt.png',
  'https://rerollcdn.com/items/NegatronCloak.png',
  'https://rerollcdn.com/items/ChaliceofPower.png',
  'https://rerollcdn.com/items/EdgeofNight.png',
  'https://rerollcdn.com/items/LocketoftheIronSolari.png',
  'https://rerollcdn.com/items/ShroudofStillness.png',
  'https://rerollcdn.com/items/ZekesHerald.png',
  'https://rerollcdn.com/items/ZzRotPortal.png',
  'https://rerollcdn.com/items/BansheesClaw.png',
  'https://rerollcdn.com/items/NegatronCloak.png',
  'https://rerollcdn.com/items/TearoftheGoddess.png',
  'https://rerollcdn.com/items/TearoftheGoddess.png',
  'https://rerollcdn.com/items/SparringGloves.png',
];

interface ItemInterface {
  guid: string;
  id: number;
  loadoutsIcon: string;
  name: string;
  nameId: string;
}

const generateItem = async (item: ItemInterface) => {
  const itemName = item.nameId.match(/(?<=_Item_)(.*)/)?.[0];

  const itemIndex = items.findIndex((image) =>
    image.includes(itemName)
  );

  await Item.create({
    ...item,
    img: itemIndex >= 0 ? items[itemIndex] : null,
    loadouts_icon: item.loadoutsIcon,
    name_id: item.nameId,
  }).save();
};

databaseConnection().then(() => {
  jsonItems.forEach((item) => generateItem(item));
});
