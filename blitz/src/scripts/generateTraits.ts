import { Trait } from '../entities/Trait';
import databaseConnection from '../index';
import jsonTraits from './assets/traits.json';

interface TraitInterface {
  display_name: string;
  icon_path: string;
  set: string;
  trait_id: string;
}

const generateTrait = async (trait: TraitInterface) => {
  await Trait.create(trait).save();
};

databaseConnection().then(() => {
  jsonTraits.forEach((trait) => generateTrait(trait));
});
