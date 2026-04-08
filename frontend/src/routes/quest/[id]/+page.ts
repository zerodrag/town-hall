import { error, redirect } from '@sveltejs/kit';
import { getQuest } from '$lib/backend/quest';
import { slugify } from '$lib/utils.js';

export const load = async ({ fetch, params }) => {
  const questResult = await getQuest(fetch, params.id);
  if (!questResult.ok) {
    error(questResult.status, questResult.body);
  }
  const quest = questResult.data;
  redirect(308, `/quest/${quest.questId}/${slugify(quest.title)}`);
};
