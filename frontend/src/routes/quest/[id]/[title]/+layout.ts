import { error, redirect } from '@sveltejs/kit';
import type { Quest } from '$lib/backend/generated-types.js';
import { getQuest } from '$lib/backend/quest.js';
import { slugify } from '$lib/utils.js';

export const load = async ({ fetch, params }) => {
  const resp = await getQuest(fetch, params.id);
  if (!resp.ok) {
    error(resp.status, await resp.text());
  }
  const quest: Quest = await resp.json();
  const slug = slugify(quest.title);
  if (slug !== params.title) {
    redirect(308, `/quest/${quest.questId}/${slug}`);
  }
  return { quest };
};
