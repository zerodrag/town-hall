import { createQuest } from '$lib/backend/quest';
import { fail, redirect } from '@sveltejs/kit';
import type { Actions } from './$types';

export const actions = {
	new: async ({ fetch, request }) => {
		const formData = await request.formData();
		const title = String(formData.get('title') ?? '')
			.trim()
			.replace(/\s+/g, ' ');

		if (title.length < 10 || title.length > 100) {
			return fail(400, { error: 'Title must be 10-100 characters long.' });
		}

		const result = await createQuest(fetch, title);

		if (!result.ok) {
			return fail(result.status, { error: result.body });
		}

		redirect(303, `/quest/${result.data}`);
	}
} satisfies Actions;
