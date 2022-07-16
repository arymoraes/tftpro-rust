<script lang="ts">
  import { onMount } from 'svelte';
  import axios from 'axios';
  import type { MatchI } from './@types/Match';
  import { Accordion, AccordionItem } from 'sveltestrap';
  import { format } from 'date-fns';

  let matches: MatchI[] = [];

  onMount(async () => {
    const res = await axios.get<[MatchI]>(
      'http://localhost:4000/matches'
    );
    matches = res.data;
    console.log({ matches });
    matches.map((match: MatchI) => {
      return {
        ...match,
        matches_participants: {
          ...match.matches_participants.sort(
            (a, b) => a.placement.valueOf() - b.placement.valueOf()
          ),
          matches_participants_units: match.matches_participants.map(
            (participant) => {
              return {
                ...participant,
                matches_participants_units_units:
                  participant.matches_participants_units.sort(
                    (a, b) => b.tier.valueOf() - a.tier.valueOf()
                  ),
              };
            }
          ),
        },
      };
    });
  });
</script>

<main>
  {#if matches.length > 0}
    <Accordion>
      {#each matches as match}
        <AccordionItem>
          <h4 class="m-0" slot="header">
            Match played at {format(
              match.game_datetime,
              'dd/MM/yyyy HH:mm'
            )} on {match.region}
          </h4>
          <Accordion>
            {#each match.matches_participants as participant}
              <AccordionItem>
                <h4 class="m-0" slot="header">
                  {participant.placement}. {participant.summoners
                    .name}
                </h4>
                <p />
                <p>
                  Level: {participant.level}
                </p>
                <p>
                  Gold Left: {participant.gold_left}
                </p>
                <div style="display: flex;">
                  {#each participant.matches_participants_units as unit}
                    <div>
                      <img src={unit.characters.img} />
                      <div>
                        {unit.tier}*
                        {unit.characters.display_name}
                      </div>
                    </div>
                  {/each}
                </div>
              </AccordionItem>
            {/each}
          </Accordion>
        </AccordionItem>
      {/each}
    </Accordion>
  {/if}
  <p>
    Visit the <a href="https://svelte.dev/tutorial">Svelte tutorial</a
    > to learn how to build Svelte apps.
  </p>
</main>

<style>
  main {
    text-align: center;
    padding: 1em;
    max-width: 240px;
    margin: 0 auto;
  }

  h1 {
    color: #ff3e00;
    text-transform: uppercase;
    font-size: 4em;
    font-weight: 100;
  }

  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }
</style>
