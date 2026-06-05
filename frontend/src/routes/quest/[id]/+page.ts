import { error, redirect } from '@sveltejs/kit';
import type { Quest } from '$lib/backend/generated-types.js';
import { getQuest } from '$lib/backend/quest';
import { slugify } from '$lib/utils.js';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch, params }) => {
  const resp = await getQuest(fetch, params.id);
  if (!resp.ok) {
    error(resp.status, await resp.text());
  }
  const quest: Quest = await resp.json();
  redirect(308, `/quest/${quest.questId}/${slugify(quest.title)}`);
};
