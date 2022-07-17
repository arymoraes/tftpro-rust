import {
  Accordion,
  Avatar,
  Badge,
  Container,
  Group,
  Tooltip,
} from '@mantine/core';
import axios, { AxiosResponse } from 'axios';
import { format } from 'date-fns';
import type { NextPage } from 'next';
import Head from 'next/head';
import Image from 'next/image';
import { useCallback, useEffect, useState } from 'react';
import { MatchI } from '../@types/Match';
import OneStar from '../assets/one-star.svg';
import ThreeStars from '../assets/three-stars.svg';
import TwoStars from '../assets/two-stars.svg';
import styles from '../styles/Home.module.css';

const colorPositionMap = {
  1: 'yellow',
  2: 'gray',
  3: 'orange',
  4: 'dark',
  5: 'indigo',
  6: 'indigo',
  7: 'indigo',
  8: 'indigo',
};

const Home: NextPage = () => {
  const [matches, setMatches] = useState<MatchI[]>([]);

  useEffect(() => {
    axios
      .get<MatchI[]>('http://localhost:4000/matches')
      .then((res: AxiosResponse<MatchI[]>) => {
        console.log({ res });
        let matches = res.data as MatchI[];

        const mappedMatches = matches.map((match: MatchI) => {
          return {
            ...match,
            matches_participants: [...match.matches_participants]
              .sort(
                (a, b) =>
                  a.placement.valueOf() - b.placement.valueOf()
              )
              .map((participant) => {
                return {
                  ...participant,
                  matches_participants_units: [
                    ...participant.matches_participants_units,
                  ].sort(
                    (a, b) => b.tier.valueOf() - a.tier.valueOf()
                  ),
                };
              })
              // remove duplicates of summoner names,
              // this is because the API returns the same summoner name twice due to a bug, but it should be fixed on the service itself
              .filter((participant, index, self) => {
                return (
                  self.findIndex(
                    (p) =>
                      p.summoners.name === participant.summoners.name
                  ) === index
                );
              }),
          };
        });
        setMatches(mappedMatches);
      });
  }, []);

  const renderAccordionLabel = useCallback(
    (name: string, position: keyof typeof colorPositionMap) => {
      return (
        <>
          <Badge
            color={colorPositionMap[position]}
            radius="sm"
            size="lg"
            sx={{
              paddingRight: 8,
              paddingLeft: 8,
              marginRight: 8,
            }}
          >
            {position}
          </Badge>
          <span>{name}</span>
        </>
      ) as JSX.Element;
    },
    []
  );

  return (
    <div className={styles.main}>
      <Head>
        <title>Create Next App</title>
        <meta
          content="Generated by create next app"
          name="description"
        />
        <link href="/favicon.ico" rel="icon" />
      </Head>

      <Container p={25} px="xl" size="xl">
        {matches.length &&
          matches.map((match: MatchI) => {
            return (
              <Accordion key={match.game_datetime.toString()}>
                <Accordion.Item
                  label={`Game played at ${format(
                    match.game_datetime,
                    'dd/MM/yyyy HH:mm'
                  )} on ${match.region}`}
                >
                  {match.matches_participants.length &&
                    match.matches_participants.map(
                      (participant, index) => {
                        return (
                          <Accordion
                            key={`${participant.summoners.name}-${index}`}
                          >
                            <Accordion.Item
                              label={renderAccordionLabel(
                                participant.summoners.name,
                                participant.placement as keyof typeof colorPositionMap
                              )}
                            >
                              <Group>
                                {participant.matches_participants_units.map(
                                  (unit) => (
                                    <>
                                      <Tooltip
                                        closeDelay={50}
                                        label={
                                          unit.characters.display_name
                                        }
                                        openDelay={50}
                                        withArrow
                                      >
                                        <Avatar
                                          key={
                                            unit.characters
                                              .display_name
                                          }
                                          radius="xs"
                                          size="lg"
                                          src={unit.characters.img}
                                        />
                                        {unit.tier === 3 ? (
                                          <Group
                                            sx={{
                                              display: 'flex',
                                              alignItems: 'center',
                                              justifyContent:
                                                'center',
                                            }}
                                          >
                                            <Image
                                              alt="Three stars"
                                              src={ThreeStars}
                                            />
                                          </Group>
                                        ) : unit.tier === 2 ? (
                                          <Group
                                            sx={{
                                              display: 'flex',
                                              alignItems: 'center',
                                              justifyContent:
                                                'center',
                                            }}
                                          >
                                            <Image
                                              alt="Two stars"
                                              src={TwoStars}
                                            />
                                          </Group>
                                        ) : (
                                          <Group
                                            sx={{
                                              display: 'flex',
                                              alignItems: 'center',
                                              visibility: 'hidden',
                                              justifyContent:
                                                'center',
                                            }}
                                          >
                                            <Image
                                              alt="one stars"
                                              src={OneStar}
                                            />
                                          </Group>
                                        )}
                                      </Tooltip>
                                    </>
                                  )
                                )}
                              </Group>
                            </Accordion.Item>
                          </Accordion>
                        );
                      }
                    )}
                </Accordion.Item>
              </Accordion>
            );
          })}
      </Container>

      <footer className={styles.footer}></footer>
    </div>
  );
};

export default Home;
