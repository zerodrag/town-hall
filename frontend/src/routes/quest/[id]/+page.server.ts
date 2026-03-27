import { getQuest } from '$lib/backend/quest';
import { error, redirect } from '@sveltejs/kit';

export const load = async ({ fetch, params }) => {
	const questResult = await getQuest(fetch, params.id);
	if (!questResult.ok) {
		error(questResult.status, questResult.body);
	}
	const quest = questResult.data;
	const slug = quest.title
		.toLowerCase()
		.trim()
		.replace(/[^\w\s-]/g, '')
		.replace(/[\s_-]+/g, '-')
		.replace(/^-+|-+$/g, '');
	redirect(301, `/quest/${quest.quest_id}/${slug}`);
};
