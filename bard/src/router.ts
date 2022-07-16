import express from 'express';
import { prismaClient } from '.';

const router = express.Router();

router.get('/matches', async (req, res) => {
  const test = await prismaClient.matches.findMany({
    orderBy: {
      game_datetime: 'desc',
    },
    select: {
      game_datetime: true,
      matches_participants: {
        select: {
          gold_left: true,
          level: true,
          matches_participants_augments: {
            select: {
              augment_id: false,
              id: false,
              items: {
                select: {
                  name_id: true,
                },
              },
            },
          },
          matches_participants_traits: {
            select: {
              style: true,
              tier_current: true,
              tier_total: true,
              traits: {
                select: {
                  display_name: true,
                },
              },
            },
          },
          matches_participants_units: {
            select: {
              characters: {
                select: {
                  display_name: true,
                  img: true,
                  tier: true,
                },
              },
              tier: true,
            },
          },
          placement: true,
          summoners: {
            select: {
              name: true,
            },
          },
        },
      },
      region: true,
    },
    take: 10,
  });
  res.json(test);
});

export default router;
